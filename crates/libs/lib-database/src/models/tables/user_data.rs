use mongodb::{
   bson::{doc, oid::ObjectId, to_bson},
   Collection,
};
use serde::{Deserialize, Serialize};

use crate::{
   error::{Error, Result},
   models::database::DatabaseManager,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
   _id: ObjectId,
   owner: ObjectId,
   task_list: Vec<Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
   pub _id: ObjectId,
   pub title: String,
   pub description: String,
   pub status: TaskStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
   NotStarted,
   Ongoing,
   Completed,
}

#[derive(Clone)]
pub struct UserDataTable {
   user_data_collection: Collection<UserData>,
}

impl UserDataTable {
   pub fn from(database_manager: DatabaseManager) -> Self {
      UserDataTable {
         user_data_collection: database_manager.users_data_collection(),
      }
   }

   pub async fn create_task(&self, user_id: ObjectId, task: Task) -> Result<()> {
      if (self
         .user_data_collection
         .find_one(doc! {"owner": user_id }, None)
         .await?)
         .is_none()
      {
         let user_data = UserData {
            _id: ObjectId::new(),
            owner: user_id,
            task_list: Vec::new(),
         };

         self
            .user_data_collection
            .insert_one(user_data.clone(), None)
            .await?;
      };

      let update = doc! {
         "$push": doc! {
            "task_list": to_bson(&task).unwrap()
         }
      };

      self
         .user_data_collection
         .update_one(doc! {"owner": user_id}, update, None)
         .await?;

      Ok(())
   }

   pub async fn get_task_list(&self, user_id: ObjectId) -> Result<Vec<Task>> {
      let filter = doc! { "owner": user_id };

      let user_data = match self.user_data_collection.find_one(filter, None).await? {
         Some(user_data) => user_data,
         None => return Err(Error::UnableToFindUserData),
      };

      println!("{:?}", user_data);

      Ok(user_data.task_list)
   }

   pub async fn update_task(&self, user_id: ObjectId, task: Task) -> Result<()> {
      let filter = doc! {"owner": user_id, "task_list._id": task._id };
      let update = doc! {"$set": doc! { "task_list.$": to_bson(&task)? } };

      self
         .user_data_collection
         .update_one(filter, update, None)
         .await?;

      Ok(())
   }

   pub async fn update_task_list(&self, user_id: ObjectId, list: Vec<Task>) -> Result<()> {
      let filter = doc! { "owner": user_id };
      let update = doc! { "$set": doc! { "list": to_bson(&list)? } };

      self
         .user_data_collection
         .update_one(filter, update, None)
         .await?;

      Ok(())
   }

   pub async fn delete_task(&self, user_id: ObjectId, task_id: ObjectId) -> Result<()> {
      let filter = doc! {"owner": user_id};
      let update = doc! {"$pull": doc! {"task_list": doc! {"_id": task_id}}};

      self
         .user_data_collection
         .update_one(filter, update, None)
         .await?;

      Ok(())
   }
}
