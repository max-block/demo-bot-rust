use bson::Document;
use chrono::{Duration, Utc};
use mongodb::{
    bson::{self, doc, Bson},
    options::FindOptions,
    results::UpdateResult,
    sync::Client,
    sync::Collection,
};
use url::Url;

use crate::{error::CoreError, models::Bot, models::Worker, models::WorkerStatus};

pub struct CreateWorkerParams {
    pub name: String,
    pub source: String,
    pub proxy: Option<String>,
}

pub struct DB {
    bot_col: Collection,
    worker_col: Collection,
}

impl DB {
    pub fn new(database_url: &str) -> Result<DB, CoreError> {
        let client = Client::with_uri_str(database_url)?;
        let database_name = parse_database_name(database_url)?;
        let bot_col = client.database(&database_name).collection("bot");
        let worker_col = client.database(&database_name).collection("worker");
        let db = DB { bot_col, worker_col };
        db.init_bot()?;

        Ok(db)
    }

    fn init_bot(&self) -> Result<(), CoreError> {
        if self.bot_col.count_documents(doc! {"_id": 1}, None)? == 0 {
            self.bot_col.insert_one(bson::to_document(&Bot::default())?, None)?;
        }
        Ok(())
    }

    pub fn get_bot(&self) -> Result<Bot, CoreError> {
        let res = self
            .bot_col
            .find_one(doc! {"_id": 1}, None)?
            .ok_or(CoreError::BotDbNotInit)?;
        Ok(bson::from_document::<Bot>(res)?)
    }

    pub fn update_bot(&self, updated: Document) -> Result<UpdateResult, CoreError> {
        Ok(self.bot_col.update_one(doc! {"_id": 1}, doc! {"$set": updated}, None)?)
    }

    pub fn insert_worker(&self, worker: CreateWorkerParams) -> Result<Worker, CoreError> {
        if self.worker_col.count_documents(doc! {"name": &worker.name}, None)? > 0 {
            return Err(CoreError::WorkerNameExists);
        }
        let res = self.worker_col.insert_one(
            bson::to_document(&Worker::new(worker.name, worker.source, worker.proxy))?,
            None,
        )?;

        let new_id = res.inserted_id.as_object_id().ok_or(CoreError::Error)?;

        // TODO: Add a new variant of CoreError
        let res = self
            .worker_col
            .find_one(doc! {"_id": new_id}, None)?
            .ok_or(CoreError::Error)?;
        Ok(bson::from_document(res)?)
    }

    pub fn find_workers(
        &self,
        status: Option<WorkerStatus>,
        has_proxy: Option<bool>,
        limit: i64,
    ) -> Result<Vec<Worker>, CoreError> {
        let mut filter = Document::new();
        if let Some(status) = status {
            filter.insert("status", status);
        }
        if let Some(has_proxy) = has_proxy {
            filter.insert(
                "proxy",
                if has_proxy {
                    doc! {"$ne":Bson::Null}
                } else {
                    doc! {"$eq":Bson::Null}
                },
            );
        }
        let options = FindOptions::builder()
            .sort(doc! { "created_at": -1})
            .limit(limit)
            .build();
        let res = self.worker_col.find(filter, options)?.collect::<Result<Vec<_>, _>>()?;
        Ok(res.into_iter().map(|d| bson::from_document(d).unwrap()).collect())
    }

    pub fn find_workers_for_work(&self, work_interval: i64, limit: i64) -> Result<Vec<Worker>, CoreError> {
        let options = FindOptions::builder()
            .sort(doc! { "last_work_at": 1})
            .limit(limit)
            .build();

        let filter = doc! {"status": WorkerStatus::Started,
        "$or": [
                {"last_work_at": Bson::Null},
                {"last_work_at": {"$lt": Utc::now() - Duration::seconds(work_interval)}}
            ]};
        let res = self.worker_col.find(filter, options)?.collect::<Result<Vec<_>, _>>()?;
        Ok(res.into_iter().map(|d| bson::from_document(d).unwrap()).collect())
    }
}

fn parse_database_name(database_url: &str) -> Result<String, CoreError> {
    let parsed = Url::parse(database_url)?;
    let database_name = parsed.path()[1..].to_string();
    if database_name.is_empty() {
        Err(CoreError::DatabaseName)
    } else {
        Ok(database_name)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_database_name() {
        let res = parse_database_name("mongodb://localhost/test");
        assert_eq!(res.unwrap(), "test");

        let res = parse_database_name("mongodb/localhost/test");
        assert!(res.is_err());

        let res = parse_database_name("mongodb://localhost/");
        assert!(res.is_err());
    }
}
