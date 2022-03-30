use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthenticationRequest {
    pub username: String,
    pub password: String,
}
