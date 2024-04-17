use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;

use crate::models::schemas::{TaskGroup, User};
use crate::response::error::Error;
use crate::response::Result;
use crate::utils::jwt::Claims;
use crate::utils::mongo::database;

pub async fn create_group(owner_id: ObjectId) -> Result {
   let task_group = TaskGroup {
      id: ObjectId::new(),
      owner: owner_id,
      list: Vec::new(),
   };

   todo!()
}

pub async fn create(claims: Claims) -> Result {
   let mongodb = database().await?;
   let users_collection = mongodb.collection::<User>("users");
   let tasks_collection = mongodb.collection::<TaskGroup>("tasks");

   let user = match users_collection
      .find_one(doc! { "username": claims.username }, None)
      .await
   {
      Ok(Some(user)) => user,
      Ok(None) => return Err(Error::MongoDBUserNotFound),
      Err(_) => return Err(Error::MongoDBFail),
   };

   match tasks_collection
      .find_one(doc! {"owner": user.id}, None)
      .await
   {
      Ok(Some(task)) => {},
      Ok(None) => {
         // Automatically create taskgroup.
         let task_group = TaskGroup {
            id: ObjectId::new(),
            owner: user.id,
            list: Vec::new(),
         };

         match tasks_collection.insert_one(task_group, None).await {
            Ok(_) => {},
            Err(_) => return Err(Error::MongoDBInsert),
         };
      },
      Err(_) => return Err(Error::MongoDBFail),
   };

   todo!()
}
