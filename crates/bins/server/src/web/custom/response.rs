use axum::{
   http::StatusCode,
   response::{IntoResponse, Response},
   Json,
};
use serde_json::{json, Map, Value};

pub struct ApiResponse {
   pub message: Option<String>,
   pub status_code: StatusCode,
   pub data: Option<Value>,
}

impl IntoResponse for ApiResponse {
   fn into_response(self) -> Response {
      let ok = self.status_code < StatusCode::from_u16(400).unwrap();

      let mut body = Map::new();
      body.insert("ok".to_string(), json!(ok));

      if let Some(message) = self.message {
         body.insert("error".to_string(), json!(message));
      }

      if let Some(data) = self.data {
         body.insert("success".to_string(), json!(data));
      }

      let body = Json(Value::Object(body));

      println!("here");

      (self.status_code, body).into_response()
   }
}
