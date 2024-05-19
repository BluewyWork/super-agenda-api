use std::str::FromStr;

use lib_database::models::{
   database::DatabaseManager,
   tables::{
      user::UserTable,
      user_data::{self, Task, TaskStatus, UserDataTable},
   },
};
use lib_utils::constants::{MONGO_DB, MONGO_URI};
use mongodb::{
   bson::{oid::ObjectId, DateTime},
   options::ClientOptions,
   results::InsertOneResult,
   Client,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
   let database_manager = DatabaseManager::from(&MONGO_URI, &MONGO_DB).await.unwrap();

   let user_table = UserTable::from(database_manager.clone());

   let user = user_table
      .find_user_from_object_id(ObjectId::from_str("6622b619c23a20b74fee6c57").unwrap())
      .await
      .unwrap();

   println!("{}", user.username);

   let user_data_table = UserDataTable::from(database_manager);

   user_data_table
      .create_task(
         ObjectId::from_str("6622b619c23a20b74fee6c57").unwrap(),
         Task {
            _id: ObjectId::new(),
            title: String::from("New Title"),
            description: String::from("This is a new description."),
            status: TaskStatus::NotStarted,
            end_date_time: DateTime::now(),
            start_date_time: DateTime::now(),
         },
      )
      .await
      .unwrap();

   let task_list = user_data_table
      .get_task_list(ObjectId::from_str("6622b619c23a20b74fee6c57").unwrap())
      .await
      .unwrap();

   println!("{:?}", task_list);

   for task in &task_list {
      println!("{}, {:?}", task.title, task);
   }

   println!("{}", task_list.len());
}
