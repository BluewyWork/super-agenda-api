use std::env;

use dotenvy::dotenv;
use lazy_static::lazy_static;

lazy_static! {
   pub static ref SERVER_ADDRESS: String = get_server_adress();
   pub static ref MONGO_URL: String = get_mongo_url();
   pub static ref MONGO_DB: String = get_mongo_db();
   pub static ref JWT_SECRET: String = get_jwt_secret();
}

fn get_server_adress() -> String {
   dotenv().ok();

   env::var("SERVER_ADDRESS").unwrap()
}

fn get_mongo_url() -> String {
   dotenv().ok();

   env::var("MONGO_URL").unwrap()
}

fn get_mongo_db() -> String {
   dotenv().ok();

   env::var("MONGO_DB").unwrap()
}

fn get_jwt_secret() -> String {
   dotenv().ok();

   env::var("JWT_SECRET").unwrap()
}
