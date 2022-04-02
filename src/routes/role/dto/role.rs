use serde::{Deserialize, Serialize};

use crate::routes::permission::dto::permission::Permission as PermissionDto;

#[derive(Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: String,
    pub permissions: Vec<PermissionDto>,
}
