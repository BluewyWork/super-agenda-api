use std::str::FromStr;

use lib_database::models::{
   database::DatabaseManager,
   tables::{
      admin::{Admin, AdminTable},
      user::UserTable,
      user_data::{Task, TaskStatus, UserDataTable},
   },
};
use lib_utils::constants::{MONGO_DB, MONGO_URI};
use mongodb::bson::{oid::ObjectId, DateTime};

#[tokio::main]
async fn main() {
   let database_manager = DatabaseManager::from(&MONGO_URI, &MONGO_DB).await.unwrap();

   let admin_table = AdminTable::from(database_manager);

   admin_table
      .create_admin(Admin {
         id: ObjectId::new(),
         username: String::from("admin"),
         hashed_password: hash_password("admin"),
      })
      .await
      .unwrap();

   let admin = admin_table.find_admin_from_username("admin").await.unwrap();

   println!("{}, {}", admin.username, admin.hashed_password);

   // let user_table = UserTable::from(database_manager.clone());
   //
   // let user = user_table
   //    .find_user_from_object_id(ObjectId::from_str("6622b619c23a20b74fee6c57").unwrap())
   //    .await
   //    .unwrap();
   //
   // println!("{}", user.username);
   //
   // let user_data_table = UserDataTable::from(database_manager);
   //
   // user_data_table
   //    .create_task(
   //       ObjectId::from_str("6622b619c23a20b74fee6c57").unwrap(),
   //       Task {
   //          _id: ObjectId::new(),
   //          title: String::from("New Title"),
   //          description: String::from("This is a new description."),
   //          status: TaskStatus::NotStarted,
   //          end_date_time: Some(DateTime::now()),
   //          start_date_time: Some(DateTime::now()),
   //       },
   //    )
   //    .await
   //    .unwrap();
   //
   // let task_list = user_data_table
   //    .get_task_list(ObjectId::from_str("6622b619c23a20b74fee6c57").unwrap())
   //    .await
   //    .unwrap();
   //
   // println!("{:?}", task_list);
   //
   // for task in &task_list {
   //    println!("{}, {:?}", task.title, task);
   // }
   //
   // println!("{}", task_list.len());
}

pub fn hash_password(password: &str) -> String {
   bcrypt::hash(password, 10).unwrap()
}
