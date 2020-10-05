use bson::Document;
use mongodb::{
    bson::{self, doc},
    results::UpdateResult,
    sync::Client,
    sync::Collection,
};
use url::Url;

use crate::{error::CoreError, models::Bot};

pub struct DB {
    bot_col: Collection,
}

impl DB {
    pub fn new(database_url: &str) -> Result<DB, CoreError> {
        let client = Client::with_uri_str(database_url)?;
        let database_name = parse_database_name(database_url)?;
        let bot_col = client.database(&database_name).collection("bot");
        let db = DB { bot_col };
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
            .ok_or(CoreError::BotDbNotInitError)?;
        Ok(bson::from_document::<Bot>(res)?)
    }

    pub fn update_bot(&self, updated: Document) -> Result<UpdateResult, CoreError> {
        Ok(self.bot_col.update_one(doc! {"_id": 1}, doc! {"$set": updated}, None)?)
    }
}

fn parse_database_name(database_url: &str) -> Result<String, CoreError> {
    let parsed = Url::parse(database_url)?;
    let database_name = parsed.path()[1..].to_string();
    if database_name.is_empty() {
        Err(CoreError::DatabaseNameError)
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
