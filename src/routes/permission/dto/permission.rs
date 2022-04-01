use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Permission {
    pub id: String,
    pub name: String,
    pub description: String,
}
