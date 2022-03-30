use mongodb::{error::Error, Database};

use crate::persistence::permission::{
    model::permission::Permission, permission_repository::PermissionRepository,
};

#[derive(Clone)]
pub struct PermissionService {
    pub repository: PermissionRepository,
}

impl PermissionService {
    pub fn new(repository: PermissionRepository) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        permission: Permission,
        db: &Database,
    ) -> Result<Option<Permission>, Error> {
        self.repository.create(permission, db).await
    }

    pub async fn find_all(&self, db: &Database) -> Result<Vec<Permission>, Error> {
        self.repository.find_all(db).await
    }

    pub async fn find_by_uuid(
        &self,
        db: &Database,
        uuid: &str,
    ) -> Result<Option<Permission>, Error> {
        self.repository.find_by_uuid(db, uuid).await
    }

    pub async fn find_by_name(
        &self,
        db: &Database,
        name: &str,
    ) -> Result<Option<Permission>, Error> {
        self.repository.find_by_name(db, name).await
    }

    pub async fn update(
        &self,
        db: &Database,
        uuid: &str,
        permission: Permission,
    ) -> Result<Option<Permission>, Error> {
        self.repository.update(db, uuid, permission).await
    }

    pub async fn delete(&self, db: &Database, uuid: &str) -> Result<u64, Error> {
        self.repository.delete(db, uuid).await
    }
}
