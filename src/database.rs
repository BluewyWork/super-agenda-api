use mongodb::{options::ClientOptions, Client, Database};

pub async fn mongodb_connection() -> Result<Database, ()> {
   let env_address = match std::env::var("MONGO_URL") {
      Ok(env) => env,
      Err(_) => return Err(()),
   };

   let client_options = match ClientOptions::parse(env_address).await {
      Ok(opt) => opt,
      Err(_) => return Err(()),
   };

   let client = match Client::with_options(client_options) {
      Ok(client) => client,
      Err(_) => return Err(()),
   };

   let env_database = match std::env::var("MONGO_DB") {
      Ok(env) => env,
      Err(_) => return Err(()),
   };
   let db = client.database(&env_database);

   Ok(db)
}
