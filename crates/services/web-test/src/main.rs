use lib_database::{user_table::UserTable, DatabaseManager};
use lib_utils::constants::{MONGO_DB, MONGO_URI};

#[tokio::main]
async fn main() {
   let database_manager = DatabaseManager::from(&MONGO_URI, &MONGO_DB).await.unwrap();

   let user_table = UserTable::from(database_manager);

   let _ = user_table.create_user("test3", "test-3-not-hashed").await;

   // let user = user_table.find_user("test").await.unwrap().unwrap();

   // println!("{}", user.password);
}
