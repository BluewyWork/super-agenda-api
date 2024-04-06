use mongodb::{options::ClientOptions, Client, Database};

use crate::utils::{
   constants::{MONGO_DB, MONGO_URL},
   log::plog,
};

pub async fn mongodb_connection() -> Result<Database, ()> {
   let client_options = match ClientOptions::parse(MONGO_URL.to_string()).await {
      Ok(opt) => opt,
      Err(err) => {
         plog(err.kind.to_string(), "mongodb".to_string(), true);
         return Err(());
      },
   };

   let client = match Client::with_options(client_options) {
      Ok(client) => client,
      Err(err) => {
         plog(err.kind.to_string(), "mongodb".to_string(), true);
         return Err(());
      },
   };

   let db = client.database(&MONGO_DB);

   Ok(db)
}
