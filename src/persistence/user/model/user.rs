use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub id: String,
    pub username: String,
    #[serde(rename(serialize = "emailAddress", deserialize = "emailAddress"))]
    pub email_address: String,
    pub password: String,
    #[serde(rename(serialize = "firstName", deserialize = "firstName"))]
    pub first_name: String,
    #[serde(rename(serialize = "lastName", deserialize = "lastName"))]
    pub last_name: String,
    pub enabled: bool,
    pub roles: Vec<String>,
    #[serde(rename(serialize = "createdAt", deserialize = "createdAt"))]
    pub created_at: String,
    #[serde(rename(serialize = "lastActive", deserialize = "lastActive"))]
    pub last_active: String,
}
