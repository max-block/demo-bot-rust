use serde::{Deserialize, Serialize};
use std::rc::Rc;

use mongodb::bson::{self, doc};

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
    pub fn new(db: Rc<DB>) -> Result<BotService, CoreError> {
        let bot = db.get_bot()?;
        Ok(BotService { db, bot })
    }

    pub fn start(&mut self) -> Result<(), CoreError> {
        self.db.update_bot(doc! {"bot_started": true})?;
        self.bot = self.db.get_bot()?;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), CoreError> {
        self.db.update_bot(doc! {"bot_started": false})?;
        self.bot = self.db.get_bot()?;
        Ok(())
    }

    pub fn update(&mut self, params: UpdateBotParams) -> Result<(), CoreError> {
        self.db.update_bot(bson::to_document(&params)?)?;
        self.bot = self.db.get_bot()?;
        Ok(())
    }
}
