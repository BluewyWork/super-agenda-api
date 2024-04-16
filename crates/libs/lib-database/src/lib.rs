pub mod error;
pub mod user_table;

use mongodb::{options::ClientOptions, Client, Collection, Database};
use user_table::User;

use crate::error::Result;

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
}
