use std::env;

use dotenvy::dotenv;
use lazy_static::lazy_static;

lazy_static! {
   pub static ref SERVER_ADDRESS: String = server_address();
   pub static ref MONGO_URI: String = mongo_uri();
   pub static ref MONGO_DB: String = mongo_db();
   pub static ref JWT_SECRET: String = jwt_secret();
}

fn server_address() -> String {
   dotenv().ok();

   env::var("SERVER_ADDRESS").unwrap()
}

fn mongo_uri() -> String {
   dotenv().ok();

   env::var("MONGO_URI").unwrap()
}

fn mongo_db() -> String {
   dotenv().ok();

   env::var("MONGO_DB").unwrap()
}

fn jwt_secret() -> String {
   dotenv().ok();

   env::var("JWT_SECRET").unwrap()
}
