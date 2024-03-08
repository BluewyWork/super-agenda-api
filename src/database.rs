use mongodb::{options::ClientOptions, Client, Database};

pub async fn mongodb_connection() -> Result<Database, ()> {
   let env_adress_name = "MONGO_URL";
   let env_address_value = match std::env::var(env_adress_name) {
      Ok(env) => env,
      Err(_) => {
         println!("env: {} not found", env_adress_name);
         return Err(());
      },
   };

   let client_options = match ClientOptions::parse(&env_address_value).await {
      Ok(opt) => opt,
      Err(_) => {
         println!("mongodb: unable to parse -> {}", env_address_value);
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

   let env_database_name = "MONGO_DB";
   let env_database_value = match std::env::var(env_database_name) {
      Ok(env) => env,
      Err(_) => {
         println!("env: {} not found", env_database_name);
         return Err(());
      },
   };
   let db = client.database(&env_database_value);

   Ok(db)
}
