use chrono::{DateTime, Utc};
use mongodb::bson::{oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bot {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub timeout: i32,              // in seconds
    pub proxy_check_interval: i32, // in seconds
    pub proxy_check_limit: i32,
    pub work_interval: i32, // in seconds
    pub work_limit: i32,    // how many workers can work at once
    pub bot_started: bool,
}

impl Default for Bot {
    fn default() -> Self {
        Bot {
            id: Some(1),
            timeout: 10,
            proxy_check_interval: 60,
            proxy_check_limit: 10,
            work_interval: 10,
            work_limit: 15,
            bot_started: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkerStatus {
    #[serde(rename = "STARTED")]
    Started,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "PROXY_ERROR")]
    ProxyError,
}

impl ToString for WorkerStatus {
    fn to_string(&self) -> String {
        match self {
            WorkerStatus::Started => String::from("STARTED"),
            WorkerStatus::Stopped => String::from("STOPPED"),
            WorkerStatus::ProxyError => String::from("PROXY_ERROR"),
        }
    }
}

impl From<WorkerStatus> for Bson {
    fn from(s: WorkerStatus) -> Self {
        Bson::String(s.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Worker {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub source: String,
    pub status: WorkerStatus,
    pub proxy: Option<String>,
    pub proxy_checked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl Worker {
    pub fn new(name: String, source: String, proxy: Option<String>) -> Worker {
        Worker {
            id: None,
            name,
            source,
            status: WorkerStatus::Stopped,
            proxy,
            proxy_checked_at: None,
            created_at: Utc::now(),
        }
    }
}
