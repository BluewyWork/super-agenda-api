use mongodb::{options::ClientOptions, Client, Collection, Database};

use crate::{
   error::Result,
   models::tables::{user::User, user_data::UserData},
};

use super::tables::admin::Admin;

#[derive(Clone)]
pub struct DatabaseManager {
   database: Database,
}

impl DatabaseManager {
   pub async fn from(mongo_uri: &str, mongo_db_name: &str) -> Result<Self> {
      let client_options = ClientOptions::parse(mongo_uri).await?;
      let client = Client::with_options(client_options)?;

      let database = client.database(mongo_db_name);

      Ok(DatabaseManager { database })
   }

   pub fn users_collection(&self) -> Collection<User> {
      self.database.collection::<User>("users")
   }

   pub fn users_data_collection(&self) -> Collection<UserData> {
      self.database.collection::<UserData>("users-data")
   }

   pub fn admin_collection(&self) -> Collection<Admin> {
      self.database.collection::<Admin>("admins")
   }
}
