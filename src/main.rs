use axum::Router;
use tokio::net::TcpListener;

use crate::config::SERVER_ADDRESS;

mod config;
mod controllers;
mod database;
mod middleware;
mod models;
mod routes;
mod schemas;
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
