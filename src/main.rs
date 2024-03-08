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

#[tokio::main]
async fn main() {
   dotenvy::dotenv().expect("dotenv: unable to access .env file");

   let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("localhost:8001".to_string());
   println!("API => {}", server_address);

   let listener = TcpListener::bind(&server_address)
      .await
      .expect("tcp: unable to create tcp listener");

   let app = Router::new()
      .route("/hello", get(hello_world))
      .route("/test", get(test))
      .route("/test2", get(test2))
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

async fn test2() -> Answer {
   let mongodb = match mongodb_connection().await {
      Ok(client) => client,
      Err(_) => {
         return Answer {
            json: "".into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            ok: false,
         };
      },
   };

   if let Ok(collection) = mongodb.list_collection_names(None).await {
      println!("{:?}", collection);
   }

   Answer {
      json: "test".into(),
      status: StatusCode::ACCEPTED,
      ok: true,
   }
}
