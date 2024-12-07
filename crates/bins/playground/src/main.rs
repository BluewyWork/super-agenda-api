use database::models::{
   database::DatabaseManager,
   tables::admin::{Admin, AdminTable},
};
use mongodb::bson::oid::ObjectId;
use utils::constants::{MONGO_DB, MONGO_URI};

#[tokio::main]
async fn main() {
   create_admin_example().await;
}

pub fn hash_password(password: &str) -> String {
   bcrypt::hash(password, 10).unwrap()
}

pub async fn create_admin_example() {
   let database_manager = DatabaseManager::from(&MONGO_URI, &MONGO_DB).await.unwrap();

   let admin_table = AdminTable::from(database_manager);

   admin_table
      .create_admin(Admin {
         id: ObjectId::new(),
         username: String::from("tarriiik"),
         hashed_password: hash_password("tarriiik"),
      })
      .await
      .unwrap();

   let admin = admin_table
      .find_admin_from_username("tarriiik")
      .await
      .unwrap();

   println!("{}, {}", admin.username, admin.hashed_password);
}
