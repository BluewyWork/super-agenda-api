use axum::{
   http::StatusCode,
   response::{Html, IntoResponse},
   routing::get,
   Router,
};
use database::mongodb_connection;
use models::api::Answer;
use serde_json::json;
use tokio::net::TcpListener;

mod controllers;
mod database;
mod error;
mod models;
mod routes;
mod schemas;
mod utils;

#[tokio::main]
async fn main() {
   dotenvy::dotenv().expect("dotenv: unable to access .env file");

   let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("localhost:8001".to_string());
   println!("API => {}", server_address);

   let listener = TcpListener::bind(&server_address)
      .await
      .expect("tcp: unable to create tcp listener");

   let app = Router::new().nest("/api/user", routes::user_routes());

   axum::serve(listener, app)
      .await
      .expect("axum: something went wrong...");
}
