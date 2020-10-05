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
