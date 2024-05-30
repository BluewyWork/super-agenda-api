use mongodb::{
   bson::{doc, oid::ObjectId},
   Collection,
};
use serde::{Deserialize, Serialize};

use crate::{
   error::{Error, Result},
   models::database::DatabaseManager,
};

#[derive(Serialize, Deserialize)]
pub struct Admin {
   #[serde(rename = "id")]
   pub id: ObjectId,
   pub username: String,
   pub hashed_password: String,
}

#[derive(Clone)]
pub struct AdminTable {
   admin_collection: Collection<Admin>,
}

impl AdminTable {
   pub fn from(database_manager: DatabaseManager) -> Self {
      AdminTable {
         admin_collection: database_manager.admin_collection(),
      }
   }

   pub async fn find_admin_from_username(&self, username: &str) -> Result<Admin> {
      let admin = match self
         .admin_collection
         .find_one(doc! {"username": username}, None)
         .await?
      {
         Some(admin) => admin,
         None => return Err(Error::UnableToFindUser),
      };

      Ok(admin)
   }

   pub async fn create_admin(&self, admin: Admin) -> Result<()> {
      self.admin_collection.insert_one(admin, None).await?;

      Ok(())
   }
}