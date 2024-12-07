use std::str::FromStr;

use futures::stream::TryStreamExt;
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
pub struct UserForUpdate {
   #[serde(skip_serializing_if = "Option::is_none")]
   pub username: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
   #[serde(rename = "_id")]
   pub id: ObjectId,
   pub username: String,
   pub hashed_password: String,
}

pub struct UserTable {
   users_collection: Collection<User>,
}

impl UserTable {
   pub fn from(database_manager: DatabaseManager) -> Self {
      UserTable {
         users_collection: database_manager.users_collection(),
      }
   }

   pub async fn find_all_users(&self) -> Result<Vec<User>> {
      let mut maybe_user_list = self.users_collection.find(doc! {}).await?;

      let mut user_list: Vec<User> = Vec::new();

      while let Some(user) = maybe_user_list.try_next().await? {
         user_list.push(user);
      }

      Ok(user_list)
   }

   pub async fn find_user_from_username(&self, username: &str) -> Result<User> {
      let user = match self
         .users_collection
         .find_one(doc! {"username": username})
         .await?
      {
         Some(user) => user,
         None => return Err(Error::UnableToFindUser),
      };

      Ok(user)
   }

   pub async fn find_user_from_object_id(&self, user_id: ObjectId) -> Result<User> {
      let user = match self
         .users_collection
         .find_one(doc! {"_id": user_id})
         .await?
      {
         Some(user) => user,
         None => return Err(Error::UnableToFindUser),
      };

      Ok(user)
   }

   pub async fn create_user(&self, user: User) -> Result<()> {
      self.users_collection.insert_one(user).await?;

      Ok(())
   }

   // Ideas for implementation:
   // 1. Create custom struct for this operation.
   pub async fn update_user(&self, id: ObjectId, user_for_update: UserForUpdate) -> Result<()> {
      let filter = doc! {"_id": id};
      let update_query = doc! {"$set": to_bson(&user_for_update)?};

      self
         .users_collection
         .update_one(filter, update_query)
         .await?;

      Ok(())
   }

   pub async fn delete_user(&self, _id: ObjectId) -> Result<()> {
      self.users_collection.delete_one(doc! {"_id": _id}).await?;

      Ok(())
   }
}
