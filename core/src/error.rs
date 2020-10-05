#[derive(Debug)]
pub enum CoreError {
    MongoError(mongodb::error::Error),
    BsonSerError(mongodb::bson::ser::Error),
    BsonDeError(mongodb::bson::de::Error),
    UrlParseError(url::ParseError),
    DatabaseNameError,
    BotDbNotInitError,
}

impl From<url::ParseError> for CoreError {
    fn from(e: url::ParseError) -> Self {
        CoreError::UrlParseError(e)
    }
}

impl From<mongodb::error::Error> for CoreError {
    fn from(e: mongodb::error::Error) -> Self {
        CoreError::MongoError(e)
    }
}

impl From<mongodb::bson::ser::Error> for CoreError {
    fn from(e: mongodb::bson::ser::Error) -> Self {
        CoreError::BsonSerError(e)
    }
}

impl From<mongodb::bson::de::Error> for CoreError {
    fn from(e: mongodb::bson::de::Error) -> Self {
        CoreError::BsonDeError(e)
    }
}
