use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Role {
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub id: String,
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
}
