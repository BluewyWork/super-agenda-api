use axum::{
   extract::{rejection::JsonRejection, FromRequest},
   http::StatusCode,
   response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(Answer))]
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
   fn into_response(self) -> axum::response::Response {
      let Self(value) = self;
      axum::Json(value).into_response()
   }
}

impl From<JsonRejection> for Answer {
   fn from(rejection: JsonRejection) -> Self {
      Self::from_status_message(rejection.status(), rejection.body_text().into())
   }
}

pub struct Answer {
   pub status: StatusCode,
   pub message: String,
   pub data: Value,
}

#[allow(dead_code)]
impl Answer {
   pub fn new() -> Answer {
      Answer {
         status: StatusCode::OK,
         message: String::from(""),
         data: json!({}),
      }
   }

   pub fn from_status(status: StatusCode) -> Answer {
      Answer {
         status,
         message: String::from(""),
         data: json!({}),
      }
   }

   pub fn from_status_message(status: StatusCode, message: String) -> Answer {
      Answer {
         status,
         message,
         data: json!({}),
      }
   }

   pub fn from_status_message_data(status: StatusCode, message: String, data: Value) -> Answer {
      Answer {
         status,
         message,
         data,
      }
   }
}

impl IntoResponse for Answer {
   fn into_response(self) -> Response {
      let ok = self.status < StatusCode::from_u16(400).unwrap();

      let json = Json(json!({
         "ok": ok,
         "message": self.message,
         "data": self.data,
      }));

      (self.status, json).into_response()
   }
}
