#[derive(Debug)]
pub enum CoreError {
    Mongo(mongodb::error::Error),
    BsonSer(mongodb::bson::ser::Error),
    BsonDe(mongodb::bson::de::Error),
    UrlParse(url::ParseError),
    DatabaseName,
    BotDbNotInit,
    WorkerNameExists,
    Error,
}

impl From<url::ParseError> for CoreError {
    fn from(e: url::ParseError) -> Self {
        CoreError::UrlParse(e)
    }
}

impl From<mongodb::error::Error> for CoreError {
    fn from(e: mongodb::error::Error) -> Self {
        CoreError::Mongo(e)
    }
}

impl From<mongodb::bson::ser::Error> for CoreError {
    fn from(e: mongodb::bson::ser::Error) -> Self {
        CoreError::BsonSer(e)
    }
}

impl From<mongodb::bson::de::Error> for CoreError {
    fn from(e: mongodb::bson::de::Error) -> Self {
        CoreError::BsonDe(e)
    }
}
