use futures::TryStreamExt;
use mongodb::{bson::doc, error::Error, Database};

use super::model::permission::Permission;

#[derive(Clone)]
pub struct PermissionRepository {
    pub collection: String,
}

impl PermissionRepository {
    pub fn new(collection: &str) -> Self {
        Self {
            collection: String::from(collection),
        }
    }

    pub async fn create(
        &self,
        permission: Permission,
        db: &Database,
    ) -> Result<Option<Permission>, Error> {
        let collection = db.collection::<Permission>(&self.collection);
        let res = match collection.insert_one(permission, None).await {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let new_uuid = match res.inserted_id.as_str() {
            Some(d) => d,
            None => return Ok(None),
        };

        self.find_by_uuid(db, new_uuid).await
    }

    pub async fn find_all(&self, db: &Database) -> Result<Vec<Permission>, Error> {
        let cursor = match db
            .collection::<Permission>(&self.collection)
            .find(None, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }

    pub async fn find_by_uuid(
        &self,
        db: &Database,
        uuid: &str,
    ) -> Result<Option<Permission>, Error> {
        let filter = doc! { "_id": uuid};
        let cursor = match db
            .collection::<Permission>(&self.collection)
            .find_one(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(cursor)
    }

    pub async fn find_by_name(
        &self,
        db: &Database,
        name: &str,
    ) -> Result<Option<Permission>, Error> {
        let filter = doc! { "name": name};
        let cursor = match db
            .collection::<Permission>(&self.collection)
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
        permission: Permission,
    ) -> Result<Option<Permission>, Error> {
        let collection = db.collection::<Permission>(&self.collection);
        let filter = doc! { "_id": uuid };
        let update = doc! {
            "$set": {
                "name": permission.name,
                "description": permission.description,
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
            .collection::<Permission>(&self.collection)
            .delete_one(qry, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(cursor.deleted_count)
    }
}
