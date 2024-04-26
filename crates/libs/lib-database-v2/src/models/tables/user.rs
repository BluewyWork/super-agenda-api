use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::models::{
   error::{Error, Result},
   model_manager::{IsTable, ModelManager},
};

#[derive(Serialize, Deserialize)]
pub struct User {
   _id: ObjectId,
   username: String,
   hashed_password: String,
}

pub struct UserTable;

impl IsTable for UserTable {
   const TABLE: &'static str = "users";
}

impl UserTable {
   pub async fn create(model_manager: &ModelManager, user: User) -> Result<()> {
      let collection = model_manager.database().collection::<User>(Self::TABLE);

      collection.insert_one(user, None).await?;

      Ok(())
   }

   pub async fn read(model_manager: &ModelManager, user_id: ObjectId) -> Result<User> {
      let collection = model_manager.database().collection::<User>(Self::TABLE);

      let filter = doc! {"_id": user_id};

      let user = match collection.find_one(filter, None).await? {
         Some(user) => user,
         None => {return Err(Error::UserNotFound)}
      };

      Ok(user)
   }
}
