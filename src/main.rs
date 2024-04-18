use axum::{middleware::map_response, Router};
use tokio::net::TcpListener;

use crate::{
   middleware::{map_response_from_error, map_response_from_success},
   utils::constants::SERVER_ADDRESS,
};

mod controllers;
mod middleware;
mod models;
mod response;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
   let listener = TcpListener::bind(SERVER_ADDRESS.to_string()).await.unwrap();

   println!("Server running on {}", *SERVER_ADDRESS);

   let app = Router::new()
      .nest("/api/user", routes::user_routes())
      .layer(map_response(map_response_from_error))
      .layer(map_response(map_response_from_success));

   axum::serve(listener, app).await.unwrap();
}
