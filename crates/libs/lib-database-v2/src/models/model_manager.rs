use lib_utils::constants::{MONGO_DB, MONGO_URI};
use mongodb::{options::ClientOptions, Client, Collection, Database};

use crate::models::error::Result;

pub struct ModelManager {
   database: Database,
}

impl ModelManager {
   pub async fn new() -> Result<Self> {
      let client_options = ClientOptions::parse(MONGO_URI.to_string()).await?;
      let client = Client::with_options(client_options)?;

      let database = client.database(&MONGO_DB);

      Ok(ModelManager { database })
   }

   pub fn database(&self) -> &Database {
      &self.database
   }
}

pub trait IsTable {
   const TABLE: &'static str;
}
