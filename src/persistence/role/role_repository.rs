use futures::TryStreamExt;
use mongodb::{bson::doc, error::Error, Database};

use super::model::role::Role;

#[derive(Clone)]
pub struct RoleRepository {
    pub collection: String,
}

impl RoleRepository {
    pub fn new(collection: &str) -> Self {
        Self {
            collection: String::from(collection),
        }
    }

    pub async fn create(&self, role: Role, db: &Database) -> Result<Option<Role>, Error> {
        let collection = db.collection::<Role>(&self.collection);
        let res = match collection.insert_one(role, None).await {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let new_uuid = match res.inserted_id.as_str() {
            Some(d) => d,
            None => return Ok(None),
        };

        self.find_by_uuid(db, new_uuid).await
    }

    pub async fn find_all(&self, db: &Database) -> Result<Vec<Role>, Error> {
        let cursor = match db
            .collection::<Role>(&self.collection)
            .find(None, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }

    pub async fn find_by_uuid(&self, db: &Database, uuid: &str) -> Result<Option<Role>, Error> {
        let filter = doc! { "_id": uuid};
        let cursor = match db
            .collection::<Role>(&self.collection)
            .find_one(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(cursor)
    }

    pub async fn find_by_name(&self, db: &Database, name: &str) -> Result<Option<Role>, Error> {
        let filter = doc! { "name": name};
        let cursor = match db
            .collection::<Role>(&self.collection)
            .find_one(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(cursor)
    }

    pub async fn update(
        &self,
        db: &Database,
        uuid: &str,
        role: Role,
    ) -> Result<Option<Role>, Error> {
        let collection = db.collection::<Role>(&self.collection);
        let filter = doc! { "_id": uuid };
        let update = doc! {
            "$set": {
                "name": role.name,
                "description": role.description,
                "permissions": mongodb::bson::to_bson(&role.permissions).unwrap()
            }
        };

        let res = match collection.update_one(filter, update, None).await {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let matched_count = res.matched_count;

        if matched_count > 0 {
            self.find_by_uuid(db, uuid).await
        } else {
            Ok(None)
        }
    }

    pub async fn delete(&self, db: &Database, uuid: &str) -> Result<u64, Error> {
        let qry = doc! { "_id": uuid };
        let cursor = match db
            .collection::<Role>(&self.collection)
            .delete_one(qry, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(cursor.deleted_count)
    }
}
