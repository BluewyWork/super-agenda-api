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
pub enum Membership {
   FREE,
   PREMIUM,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
   #[serde(rename = "_id")]
   id: ObjectId,
   owner: ObjectId,
   task_list: Vec<Task>,
   membership: Membership,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDataForUpdate {
   #[serde(skip_serializing_if = "Option::is_none")]
   pub membership: Option<Membership>,
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

   async fn get_user_data(&self, user_id: ObjectId) -> Result<UserData> {
      let filter = doc! { "owner": user_id };

      match self.user_data_collection.find_one(filter).await? {
         Some(user_data) => Ok(user_data),
         None => Err(Error::UnableToFindUserData),
      }
   }

   pub async fn delete(&self, user_id: ObjectId) -> Result<()> {
      let filter = doc! { "owner": user_id };
      self.user_data_collection.delete_one(filter).await?;

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
         .update_one(doc! {"owner": user_id}, update)
         .await?;

      Ok(())
   }

   pub async fn get_task_list(&self, user_id: ObjectId) -> Result<Vec<Task>> {
      let user_data = self.get_user_data(user_id).await?;

      Ok(user_data.task_list)
   }

   pub async fn update_task(&self, user_id: ObjectId, task: Task) -> Result<()> {
      let filter = doc! {"owner": user_id, "task_list._id": task.id };
      let update = doc! {"$set": doc! { "task_list.$": to_bson(&task)? } };

      let result = self.user_data_collection.find_one(filter.clone()).await;

      match result {
         Ok(_) => {},
         Err(_) => {
            // Quick fix for importing tasks (on the android side) that might need to be created
            // because this is the first solution I thought of.
            self.create_task(user_id, task).await?;
         },
      }

      self.user_data_collection.update_one(filter, update).await?;

      Ok(())
   }

   pub async fn update_task_list(&self, user_id: ObjectId, list: Vec<Task>) -> Result<()> {
      let filter = doc! { "owner": user_id };
      let update = doc! { "$set": doc! { "task_list": to_bson(&list)? } };

      self.user_data_collection.update_one(filter, update).await?;

      Ok(())
   }

   pub async fn delete_task(&self, user_id: ObjectId, task_id: String) -> Result<()> {
      let task_id = ObjectId::from_str(&task_id)?;

      let filter = doc! {"owner": user_id};
      let update = doc! {"$pull": doc! {"task_list": doc! {"_id": task_id}}};

      self.user_data_collection.update_one(filter, update).await?;

      Ok(())
   }

   pub async fn initialize_userdata(&self, user_id: ObjectId) -> Result<()> {
      if (self
         .user_data_collection
         .find_one(doc! {"owner": user_id })
         .await?)
         .is_none()
      {
         let user_data = UserData {
            id: ObjectId::new(),
            owner: user_id,
            task_list: Vec::new(),
            membership: Membership::FREE,
         };

         self
            .user_data_collection
            .insert_one(user_data.clone())
            .await?;
      };

      Ok(())
   }

   pub async fn get_membership(&self, user_id: ObjectId) -> Result<Membership> {
      let user_data = self.get_user_data(user_id).await?;

      Ok(user_data.membership)
   }

   pub async fn update_user_data(
      &self,
      user_id: String,
      user_data_for_update: UserDataForUpdate,
   ) -> Result<()> {
      let filter = doc! {"owner": user_id };
      let update = doc! {"$set": to_bson(&user_data_for_update)? };

      self.user_data_collection.update_one(filter, update).await?;

      Ok(())
   }
}
