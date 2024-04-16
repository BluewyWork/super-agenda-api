pub mod web {
   pub mod custom_extractors;
   pub mod custom_response;
   pub mod error;
   pub mod mw_res_map;
   pub mod password_manager;
   pub mod routes_user_login;
   pub mod routes_user_register;
   pub mod token;
}

use axum::{middleware::map_response, Router};
use lib_database::{user_table::UserTable, DatabaseManager};
use lib_utils::constants::{MONGO_DB, MONGO_URI, SERVER_ADDRESS};
use tokio::net::TcpListener;

use crate::web::{
   error::Result, mw_res_map::map_response_from_error, routes_user_login, routes_user_register,
};

#[tokio::main]
async fn main() -> Result<()> {
   let database_manager = DatabaseManager::from(&MONGO_URI, &MONGO_DB).await.unwrap();
   let user_table = UserTable::from(database_manager);

   let user_routes = Router::new()
      .nest("/user", routes_user_register::routes(user_table.clone()))
      .nest("/user", routes_user_login::routes(user_table.clone()));

   let app = Router::new()
      .nest("/api", user_routes)
      .layer(map_response(map_response_from_error));

   let listener = TcpListener::bind(SERVER_ADDRESS.to_string()).await.unwrap();

   println!("Server running on {}", *SERVER_ADDRESS);

   axum::serve(listener, app).await.unwrap();

   Ok(())
}
