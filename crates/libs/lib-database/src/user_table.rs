use mongodb::{
   bson::{doc, oid::ObjectId},
   Collection,
};
use serde::{Deserialize, Serialize};

use crate::{error::{Error, Result}, DatabaseManager};

#[derive(Serialize, Deserialize)]
pub struct User {
   pub _id: ObjectId,
   pub username: String,
   pub hashed_password: String,
}

// Axum state extractor requires clone trait.
#[derive(Clone)]
pub struct UserTable {
   users_collection: Collection<User>,
}

impl UserTable {
   pub fn from(database_manager: DatabaseManager) -> Self {
      UserTable {
         users_collection: database_manager.users_collection(),
      }
   }

   pub async fn find_user(&self, username: &str) -> Result<User> {
      let user = match self
         .users_collection
         .find_one(doc! {"username": username}, None)
         .await? {
            Some(user) => user,
            None => return Err(Error::UnableToFindUser)
         };

      Ok(user)
   }

   pub async fn create_user(&self, username: &str, hashed_password: &str) -> Result<()> {
      let user = User {
         _id: ObjectId::new(),
         username: String::from(username),
         hashed_password: String::from(hashed_password),
      };
      self.users_collection.insert_one(user, None).await?;

      Ok(())
   }
}
