use mongodb::{error::Error, Database};

use crate::persistence::user::{model::user::User, user_repository::UserRepository};

#[derive(Clone)]
pub struct UserService {
    pub repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn create(&self, user: User, db: &Database) -> Result<Option<User>, Error> {
        self.repository.create(user, db).await
    }

    pub async fn find_all(&self, db: &Database) -> Result<Vec<User>, Error> {
        self.repository.find_all(db).await
    }

    pub async fn find_by_uuid(&self, db: &Database, uuid: &str) -> Result<Option<User>, Error> {
        self.repository.find_by_uuid(db, uuid).await
    }

    pub async fn find_by_username(
        &self,
        db: &Database,
        username: &str,
    ) -> Result<Option<User>, Error> {
        self.repository.find_by_username(db, username).await
    }

    pub async fn find_by_email_address(
        &self,
        db: &Database,
        email_address: &str,
    ) -> Result<Option<User>, Error> {
        self.repository
            .find_by_email_address(db, email_address)
            .await
    }

    pub async fn update(
        &self,
        db: &Database,
        uuid: &str,
        user: User,
    ) -> Result<Option<User>, Error> {
        self.repository.update(db, uuid, user).await
    }

    pub async fn update_password(
        &self,
        db: &Database,
        uuid: &str,
        password: &str,
    ) -> Result<Option<User>, Error> {
        self.repository.update_password(db, uuid, password).await
    }

    pub async fn update_last_active(
        &self,
        db: &Database,
        uuid: &str,
        last_active: &str,
    ) -> Result<Option<User>, Error> {
        self.repository
            .update_last_active(db, uuid, last_active)
            .await
    }

    pub async fn delete(&self, db: &Database, uuid: &str) -> Result<u64, Error> {
        self.repository.delete(db, uuid).await
    }
}
