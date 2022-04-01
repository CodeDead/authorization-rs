use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateRole {
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
}
