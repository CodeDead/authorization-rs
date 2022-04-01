use mongodb::{error::Error, Database};

use crate::persistence::role::{model::role::Role, role_repository::RoleRepository};

#[derive(Clone)]
pub struct RoleService {
    pub repository: RoleRepository,
}

impl RoleService {
    pub fn new(repository: RoleRepository) -> Self {
        Self { repository }
    }

    pub async fn create(&self, role: Role, db: &Database) -> Result<Option<Role>, Error> {
        self.repository.create(role, db).await
    }

    pub async fn find_all(&self, db: &Database) -> Result<Vec<Role>, Error> {
        self.repository.find_all(db).await
    }

    pub async fn find_by_uuid(&self, db: &Database, uuid: &str) -> Result<Option<Role>, Error> {
        self.repository.find_by_uuid(db, uuid).await
    }

    pub async fn find_by_name(&self, db: &Database, name: &str) -> Result<Option<Role>, Error> {
        self.repository.find_by_name(db, name).await
    }

    pub async fn update(
        &self,
        db: &Database,
        uuid: &str,
        role: Role,
    ) -> Result<Option<Role>, Error> {
        self.repository.update(db, uuid, role).await
    }

    pub async fn delete(&self, db: &Database, uuid: &str) -> Result<u64, Error> {
        self.repository.delete(db, uuid).await
    }
}
