use mongodb::{sync::Client, sync::Collection};
use url::Url;

use crate::error::CoreError;

pub struct DB {
    pub bot_col: Collection,
    pub worker_col: Collection,
}

impl DB {
    pub fn new(database_url: &str) -> Result<DB, CoreError> {
        let client = Client::with_uri_str(database_url)?;
        let database_name = parse_database_name(database_url)?;
        let bot_col = client.database(&database_name).collection("bot");
        let worker_col = client.database(&database_name).collection("worker");
        let db = DB { bot_col, worker_col };
        Ok(db)
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
