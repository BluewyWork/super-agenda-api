use dotenvy_macro::dotenv;
use lazy_static::lazy_static;

lazy_static! {
   pub static ref SERVER_ADDRESS: String = get_server_adress();
   pub static ref MONGO_URL: String = get_mongo_url();
   pub static ref MONGO_DB: String = get_mongo_db();
   pub static ref JWT_SECRET: String = get_jwt_secret();
}

fn get_server_adress() -> String {
   let server_address = dotenv!("SERVER_ADDRESS");

   server_address.to_string()
}

fn get_mongo_url() -> String {
   let mongo_url = dotenv!("MONGO_URL");

   mongo_url.to_string()
}

fn get_mongo_db() -> String {
   let mongo_db = dotenv!("MONGO_DB");

   mongo_db.to_string()
}

fn get_jwt_secret() -> String {
   let jwt_secret = dotenv!("JWT_SECRET");

   jwt_secret.to_string()
}
