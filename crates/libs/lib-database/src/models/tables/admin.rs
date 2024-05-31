use std::str::FromStr;

use futures::TryStreamExt;
use mongodb::{
   bson::{doc, oid::ObjectId, to_bson},
   Collection,
};
use serde::{Deserialize, Serialize};

use crate::{
   error::{Error, Result},
   models::database::DatabaseManager,
};

#[derive(Serialize, Deserialize)]
pub struct Admin {
   #[serde(rename = "_id")]
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

   pub async fn find_all(&self) -> Result<Vec<Admin>> {
      let mut maybe_admin_list = self.admin_collection.find(doc! {}, None).await?;

      let mut admin_list: Vec<Admin> = Vec::new();

      while let Some(admin) = maybe_admin_list.try_next().await? {
         admin_list.push(admin)
      }

      Ok(admin_list)
   }

   pub async fn create_admin(&self, admin: Admin) -> Result<()> {
      self.admin_collection.insert_one(admin, None).await?;

      Ok(())
   }

   pub async fn update_admin(&self, admin: Admin) -> Result<()> {
      let filter = doc! {"_id": admin.id};
      let update_query = doc! { "$set": to_bson(&admin)?};

      self
         .admin_collection
         .update_one(filter, update_query, None)
         .await?;

      Ok(())
   }

   pub async fn delete_admin(&self, admin_id: String) -> Result<()> {
   let admin_id = ObjectId::from_str(&admin_id)?;

      self
         .admin_collection
         .delete_one(doc! {"_id": admin_id}, None)
         .await?;

      Ok(())
   }
}
