use std::str::FromStr;

use lib_database::models::{
   database::DatabaseManager,
   tables::{
      user::UserTable,
      user_data::{self, Task, TaskStatus, UserDataTable},
   },
};
use lib_utils::constants::{MONGO_DB, MONGO_URI};
use mongodb::{bson::{oid::ObjectId, DateTime}, options::ClientOptions, results::InsertOneResult, Client};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
   // let database_manager = DatabaseManager::from(&MONGO_URI, &MONGO_DB).await.unwrap();
   //
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
   //          title: String::from("Random Title"),
   //          description: String::from("This is a random description."),
   //          status: TaskStatus::NotStarted,
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
   
let client_options = ClientOptions::parse("mongodb+srv://test:free1@cluster0.nqqnkrf.mongodb.net/").await.unwrap();
let client = Client::with_options(client_options).unwrap();
let db = client.database("super-agenda");

let collection = db.collection::<Order>("test");

let _ = collection.insert_one(Order {
   item: String::from("hi"),
   delivery_date: DateTime::now()
}, None).await;
}

// struct AppState {
//    api_state: ApiState
// }
//
// struct ApiState {
//    user_table: UserTable,
//    user_data_table: UserDataTable
// }
//
// impl FromRef<AppState> for ApiState {
//    fn from_ref(app_state: &AppState) -> ApiState {
//       app_state.api_state.clone()
//    }
// }

#[derive(Serialize, Deserialize)]
struct Order {
    item: String,
    delivery_date: DateTime,
}
