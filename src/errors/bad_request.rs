use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
pub struct BadRequest {
    message: String,
    timestamp: String,
}

impl BadRequest {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
            timestamp: Utc::now().to_string(),
        }
    }
}
