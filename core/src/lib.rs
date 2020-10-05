use std::rc::Rc;

use db::DB;
use error::CoreError;
use services::bot::BotService;

pub mod db;
pub mod error;
pub mod models;
pub mod services;

pub struct Core {
    pub db: Rc<DB>,
    pub bot: BotService,
}

impl Core {
    pub fn new(database_url: &str) -> Result<Core, CoreError> {
        let db = Rc::new(DB::new(database_url)?);
        let bot = BotService::new(Rc::clone(&db))?;

        Ok(Core { db, bot })
    }
}
