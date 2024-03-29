use axum::{
   http::StatusCode,
   response::{IntoResponse, Response},
   Json,
};
use serde_json::{json, Value};

pub struct Answer {
   pub data: Value,
   pub status: StatusCode,
   pub ok: bool,
}

impl IntoResponse for Answer {
   fn into_response(self) -> Response {
      let json = Json(json!({
         "data": self.data,
         "ok": self.ok
      }));

      (self.status, json).into_response()
   }
}
