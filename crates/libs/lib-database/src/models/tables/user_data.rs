use std::str::FromStr;

use mongodb::{
   bson::{doc, oid::ObjectId, to_bson, DateTime},
   Collection,
};
use serde::{Deserialize, Serialize};

use crate::{
   error::{Error, Result},
   models::database::DatabaseManager,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
   #[serde(rename = "_id")]
   id: ObjectId,
   owner: ObjectId,
   task_list: Vec<Task>,
   deleted_tasks: Vec<ObjectId>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
   #[serde(rename = "_id")]
   pub id: ObjectId,
   pub title: String,
   pub description: String,
   pub status: TaskStatus,
   pub start_date_time: Option<DateTime>,
   pub end_date_time: Option<DateTime>,
   pub last_modified: DateTime,
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

   // UserData related stuff.

   pub async fn delete(&self, user_id: ObjectId) -> Result<()> {
      let filter = doc! { "owner": user_id };
      self.user_data_collection.delete_one(filter, None).await?;

      Ok(())
   }

   // Task related stuff.

   pub async fn create_task(&self, user_id: ObjectId, task: Task) -> Result<()> {
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

      Ok(user_data.task_list)
   }

   pub async fn update_task(&self, user_id: ObjectId, task: Task) -> Result<()> {
      let filter = doc! {"owner": user_id, "task_list._id": task.id };
      let update = doc! {"$set": doc! { "task_list.$": to_bson(&task)? } };

      let result = self
         .user_data_collection
         .find_one(filter.clone(), None)
         .await;

      match result {
         Ok(_) => {},
         Err(_) => {
            // Quick fix for importing tasks (on the android side) that might need to be created
            // because this is the first solution I thought of.
            self.create_task(user_id, task).await?;
         },
      }

      self
         .user_data_collection
         .update_one(filter, update, None)
         .await?;

      Ok(())
   }

   pub async fn update_task_list(&self, user_id: ObjectId, list: Vec<Task>) -> Result<()> {
      let filter = doc! { "owner": user_id };
      let update = doc! { "$set": doc! { "task_list": to_bson(&list)? } };

      self
         .user_data_collection
         .update_one(filter, update, None)
         .await?;

      Ok(())
   }

   pub async fn delete_task(&self, user_id: ObjectId, task_id: String) -> Result<()> {
      let task_id = ObjectId::from_str(&task_id)?;

      let filter = doc! {"owner": user_id};
      let update = doc! {"$pull": doc! {"task_list": doc! {"_id": task_id}}};

      self
         .user_data_collection
         .update_one(filter, update, None)
         .await?;

      Ok(())
   }

   pub async fn initialize_userdata(&self, user_id: ObjectId) -> Result<()> {
      if (self
         .user_data_collection
         .find_one(doc! {"owner": user_id }, None)
         .await?)
         .is_none()
      {
         let user_data = UserData {
            id: ObjectId::new(),
            owner: user_id,
            task_list: Vec::new(),
            deleted_tasks: Vec::new()
         };

         self
            .user_data_collection
            .insert_one(user_data.clone(), None)
            .await?;
      };

      Ok(())
   }

   pub async fn get_deleted_task_list(&self, user_id: ObjectId) -> Result<Vec<ObjectId>> {
      let filter = doc! { "owner": user_id };

      let user_data = match self.user_data_collection.find_one(filter, None).await? {
         Some(user_data) => user_data,
         None => return Err(Error::UnableToFindUserData),
      };

      Ok(user_data.deleted_tasks)
   }
   

   pub async fn add_deleted_task(&self, deleted_task_id: String, user_id: ObjectId) -> Result<()> {
      let deleted_task_id = ObjectId::from_str(&deleted_task_id)?;

      let update = doc! {
         "$push": doc! {
            "deleted_tasks": deleted_task_id
         }
      };

      self
         .user_data_collection
         .update_one(doc! {"owner": user_id}, update, None)
         .await?;

      Ok(())
   }
}
