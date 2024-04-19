use axum::{
   http::StatusCode,
   response::{IntoResponse, Response},
   Json,
};
use serde_json::{json, Value};

pub struct ApiResponse {
   pub message: Option<String>,
   pub status_code: StatusCode,
   pub data: Option<Value>,
}

impl IntoResponse for ApiResponse {
   fn into_response(self) -> Response {
      let ok = self.status_code < StatusCode::from_u16(400).unwrap();

      let body = Json(json!({
         "ok": ok,
         "message":self.message.unwrap_or(String::from("")),
         "data": self.data.unwrap_or(json!({}))
      }));

      (self.status_code, body).into_response()
   }
}
