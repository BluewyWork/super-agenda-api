use mongodb::{options::ClientOptions, Client, Database};

use crate::config::{MONGO_DB, MONGO_URL};

pub async fn mongodb_connection() -> Result<Database, ()> {
   let client_options = match ClientOptions::parse(MONGO_URL.to_string()).await {
      Ok(opt) => opt,
      Err(_) => {
         println!("mongodb: unable to parse -> {}", MONGO_URL.to_string());
         return Err(());
      },
   };

   let client = match Client::with_options(client_options) {
      Ok(client) => client,
      Err(_) => {
         println!("mongodb: unable to obtain client");
         return Err(());
      },
   };

   let db = client.database(&MONGO_DB);

   Ok(db)
}
