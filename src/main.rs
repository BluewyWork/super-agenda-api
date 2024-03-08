use axum::{
   http::StatusCode,
   response::{Html, IntoResponse},
   routing::get,
   Router,
};
use models::api::Answer;
use serde_json::json;
use tokio::net::TcpListener;

mod controllers;
mod error;
mod models;
mod routes;

#[tokio::main]
async fn main() {
   dotenvy::dotenv().expect("dotenv: unable to access .env file");

   let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("localhost:8001".to_owned());
   println!("API => {}", server_address);

   let database_url = std::env::var("DATABASE_URL").unwrap_or("localhost:8002".to_owned());
   println!("DATABASE => {}", database_url);

   let listener = TcpListener::bind(&server_address)
      .await
      .expect("tcp: unable to create tcp listener");

   let app = Router::new()
      .route("/hello", get(hello_world))
      .route("/test", get(test))
      .merge(routes::auth_user());

   axum::serve(listener, app)
      .await
      .expect("axum: something went wrong...");
}

async fn hello_world() -> impl IntoResponse {
   Html("Hello World")
}

async fn test() -> Answer {
   let json = json!({
      "test": "this is test content"
   });

   Answer {
      json,
      status: StatusCode::ACCEPTED,
      ok: true,
   }
}
