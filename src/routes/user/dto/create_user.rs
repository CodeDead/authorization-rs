use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateUser {
    pub username: String,
    #[serde(rename(serialize = "emailAddress", deserialize = "emailAddress"))]
    pub email_address: String,
    pub password: String,
    #[serde(rename(serialize = "firstName", deserialize = "firstName"))]
    pub first_name: String,
    #[serde(rename(serialize = "lastName", deserialize = "lastName"))]
    pub last_name: String,
    pub roles: Vec<String>,
}
