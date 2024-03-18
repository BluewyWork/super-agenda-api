use axum::Router;
use tokio::net::TcpListener;

use crate::utils::config::SERVER_ADDRESS;

mod controllers;
mod middleware;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
   let listener = TcpListener::bind(SERVER_ADDRESS.to_string())
      .await
      .expect("tcp: unable to create tcp listener");

   println!("API => {}", SERVER_ADDRESS.to_string());

   let app = Router::new().nest("/api/user", routes::user_routes());

   axum::serve(listener, app)
      .await
      .expect("axum: something went wrong...");
}
