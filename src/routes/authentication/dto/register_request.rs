use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
    #[serde(rename(deserialize = "firstName"))]
    pub first_name: String,
    #[serde(rename(deserialize = "lastName"))]
    pub last_name: String,
    pub username: String,
    #[serde(rename(deserialize = "emailAddress"))]
    pub email_address: String,
    pub password: String,
}
