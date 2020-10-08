use chrono::{Duration, Utc};
use mongodb::{
    bson::{self, doc, Bson, Document},
    options::FindOptions,
};
use std::rc::Rc;

use crate::{db::DB, error::CoreError, models::Worker, models::WorkerStatus};

pub struct CreateWorkerParams {
    pub name: String,
    pub source: String,
    pub proxy: Option<String>,
}

pub struct WorkerService {
    db: Rc<DB>,
}

impl WorkerService {
    pub fn new(db: Rc<DB>) -> Self {
        Self { db }
    }

    pub fn create(&self, worker: CreateWorkerParams) -> Result<Worker, CoreError> {
        if self.db.worker_col.count_documents(doc! {"name": &worker.name}, None)? > 0 {
            return Err(CoreError::WorkerNameExists);
        }
        let res = self.db.worker_col.insert_one(
            bson::to_document(&Worker::new(worker.name, worker.source, worker.proxy))?,
            None,
        )?;

        let new_id = res.inserted_id.as_object_id().ok_or(CoreError::Error)?;

        // TODO: Add a new variant of CoreError
        let res = self
            .db
            .worker_col
            .find_one(doc! {"_id": new_id}, None)?
            .ok_or(CoreError::Error)?;
        Ok(bson::from_document(res)?)
    }

    pub fn find(
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
        let res = self
            .db
            .worker_col
            .find(filter, options)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(res.into_iter().map(|d| bson::from_document(d).unwrap()).collect())
    }

    pub fn find_work(&self, work_interval: i64, limit: i64) -> Result<Vec<Worker>, CoreError> {
        let options = FindOptions::builder()
            .sort(doc! { "last_work_at": 1})
            .limit(limit)
            .build();

        let filter = doc! {"status": WorkerStatus::Started,
        "$or": [
                {"last_work_at": Bson::Null},
                {"last_work_at": {"$lt": Utc::now() - Duration::seconds(work_interval)}}
            ]};
        let res = self.db.worker_col.find(filter, options)?.collect::<Result<Vec<_>, _>>()?;
        Ok(res.into_iter().map(|d| bson::from_document(d).unwrap()).collect())
    }
}
