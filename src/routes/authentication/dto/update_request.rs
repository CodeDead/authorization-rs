use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateRequest {
    pub username: String,
    #[serde(rename(serialize = "emailAddress", deserialize = "emailAddress"))]
    pub email_address: String,
    #[serde(rename(serialize = "firstName", deserialize = "firstName"))]
    pub first_name: String,
    #[serde(rename(serialize = "lastName", deserialize = "lastName"))]
    pub last_name: String,
}
