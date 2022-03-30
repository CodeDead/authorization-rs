use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
pub struct InternalServerError {
    message: String,
    timestamp: String,
    #[serde(rename(serialize = "errorCode", deserialize = "errorCode"))]
    error_code: u16,
}

impl InternalServerError {
    pub fn new(message: &str) -> InternalServerError {
        InternalServerError {
            message: String::from(message),
            timestamp: Utc::now().to_string(),
            error_code: 500,
        }
    }
}
