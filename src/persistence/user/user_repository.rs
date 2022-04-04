use futures::TryStreamExt;
use mongodb::{bson::doc, error::Error, Database};

use super::model::user::User;

#[derive(Clone)]
pub struct UserRepository {
    pub collection: String,
}

impl UserRepository {
    pub fn new(collection: &str) -> Self {
        Self {
            collection: String::from(collection),
        }
    }

    pub async fn create(&self, user: User, db: &Database) -> Result<Option<User>, Error> {
        let collection = db.collection::<User>(&self.collection);
        let res = match collection.insert_one(user, None).await {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let new_uuid = match res.inserted_id.as_str() {
            Some(d) => d,
            None => return Ok(None),
        };

        self.find_by_uuid(db, new_uuid).await
    }

    pub async fn find_all(&self, db: &Database) -> Result<Vec<User>, Error> {
        let cursor = match db
            .collection::<User>(&self.collection)
            .find(None, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }

    pub async fn find_by_uuid(&self, db: &Database, uuid: &str) -> Result<Option<User>, Error> {
        let filter = doc! { "_id": uuid};
        let cursor = match db
            .collection::<User>(&self.collection)
            .find_one(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(cursor)
    }

    pub async fn find_by_username(
        &self,
        db: &Database,
        username: &str,
    ) -> Result<Option<User>, Error> {
        let collection = db.collection::<User>(&self.collection);
        let qry = doc! { "username": username};

        let res = match collection.find_one(qry, None).await {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(res)
    }

    pub async fn find_by_email_address(
        &self,
        db: &Database,
        email_address: &str,
    ) -> Result<Option<User>, Error> {
        let collection = db.collection::<User>(&self.collection);
        let filter = doc! { "emailAddress": email_address.to_lowercase()};

        let res = match collection.find_one(filter, None).await {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(res)
    }

    pub async fn update(
        &self,
        db: &Database,
        uuid: &str,
        user: User,
    ) -> Result<Option<User>, Error> {
        let collection = db.collection::<User>(&self.collection);
        let filter = doc! { "_id": uuid };
        let update = doc! {
            "$set": {
                "username": user.username,
                "emailAddress": user.email_address,
                "firstName": user.first_name,
                "lastName": user.last_name,
                "enabled": user.enabled,
                "roles": mongodb::bson::to_bson(&user.roles).unwrap()
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

    pub async fn update_password(
        &self,
        db: &Database,
        uuid: &str,
        password: &str,
    ) -> Result<Option<User>, Error> {
        let collection = db.collection::<User>(&self.collection);
        let filter = doc! {"_id": uuid};

        let update = doc! {"$set": {
            "password": password,
        }};

        let res = match collection.update_one(filter, update, None).await {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let count = res.matched_count;

        if count > 0 {
            self.find_by_uuid(db, uuid).await
        } else {
            Ok(None)
        }
    }

    pub async fn update_last_active(
        &self,
        db: &Database,
        uuid: &str,
        last_active: &str,
    ) -> Result<Option<User>, Error> {
        let collection = db.collection::<User>(&self.collection);
        let filter = doc! {"_id": uuid};

        let update = doc! {"$set": {
            "lastActive": last_active,
        }};

        let res = match collection.update_one(filter, update, None).await {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let count = res.matched_count;

        if count > 0 {
            self.find_by_uuid(db, uuid).await
        } else {
            Ok(None)
        }
    }

    pub async fn delete(&self, db: &Database, uuid: &str) -> Result<u64, Error> {
        let qry = doc! { "_id": uuid };
        let cursor = match db
            .collection::<User>(&self.collection)
            .delete_one(qry, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(cursor.deleted_count)
    }

    pub async fn delete_role(&self, db: &Database, role_id: &str) -> Result<u64, Error> {
        let collection = db.collection::<User>(&self.collection);
        let update = doc! { "$pull": {"roles": role_id}};
        let filter = doc! {};

        match collection.update_many(filter, update, None).await {
            Ok(d) => Ok(d.modified_count),
            Err(e) => Err(e),
        }
    }
}
