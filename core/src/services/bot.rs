use serde::{Deserialize, Serialize};
use std::rc::Rc;

use mongodb::{
    bson::{self, doc, Document},
    options::FindOneAndUpdateOptions,
    options::ReturnDocument,
};

use crate::{db::DB, error::CoreError, models::Bot};

pub struct BotService {
    db: Rc<DB>,
    pub bot: Bot,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateBotParams {
    pub timeout: i32,
    pub proxy_check_interval: i32,
    pub work_interval: i32,
}

impl BotService {
    pub fn new(db: Rc<DB>) -> Result<Self, CoreError> {
        let bot = BotService::init_bot(&db)?;
        Ok(Self { db, bot })
    }

    pub fn start(&mut self) -> Result<(), CoreError> {
        self.bot = self._update(doc! {"bot_started": true})?;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), CoreError> {
        self.bot = self._update(doc! {"bot_started": false})?;
        Ok(())
    }

    pub fn update(&mut self, params: UpdateBotParams) -> Result<(), CoreError> {
        self.bot = self._update(bson::to_document(&params)?)?;
        Ok(())
    }

    fn _update(&self, updated: Document) -> Result<Bot, CoreError> {
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();
        let res = self
            .db
            .bot_col
            .find_one_and_update(doc! {"_id": 1}, doc! {"$set": updated}, options)?
            .expect("not bot in db");
        Ok(bson::from_document::<Bot>(res)?)
    }

    fn init_bot(db: &DB) -> Result<Bot, CoreError> {
        if db.bot_col.count_documents(doc! {"_id": 1}, None)? == 0 {
            db.bot_col.insert_one(bson::to_document(&Bot::default())?, None)?;
        }
        let res = db
            .bot_col
            .find_one(doc! {"_id": 1}, None)?
            .ok_or(CoreError::BotDbNotInit)?;
        Ok(bson::from_document::<Bot>(res)?)
    }
}
