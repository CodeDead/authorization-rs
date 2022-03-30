use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdatePassword {
    pub password: String,
}
