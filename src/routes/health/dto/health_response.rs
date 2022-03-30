use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

impl HealthResponse {
    pub fn new(status: &str) -> Self {
        Self {
            status: String::from(status),
        }
    }
}
