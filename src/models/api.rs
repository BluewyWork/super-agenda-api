use axum::{
   http::StatusCode,
   response::{IntoResponse, Response},
   Json,
};
use serde_json::{json, Value};

pub struct Answer {
   pub json: Value,
   pub status: StatusCode,
   pub ok: bool,
}

impl IntoResponse for Answer {
   fn into_response(self) -> Response {
      let body = Json(json!({
         "data": self.json,
         "ok": self.ok
      }));

      (self.status, body).into_response()
   }
}
