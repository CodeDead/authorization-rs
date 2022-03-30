use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
pub struct InternalServerError {
    message: String,
    timestamp: String,
}

impl InternalServerError {
    pub fn new(message: &str) -> InternalServerError {
        InternalServerError {
            message: String::from(message),
            timestamp: Utc::now().to_string(),
        }
    }
}
