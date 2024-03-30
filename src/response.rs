use axum::{
   http::StatusCode,
   response::{IntoResponse, Response},
   Json,
};
use serde_json::{json, Value};

pub struct Answer {
   pub status: StatusCode,
   pub message: String,
   pub data: Value,
   pub error: String,
}

impl Answer {
   pub fn new() -> Answer {
      Answer {
         status: StatusCode::OK,
         message: String::from(""),
         data: json!({}),
         error: String::from(""),
      }
   }

   pub fn from_status(status: StatusCode) -> Answer {
      Answer {
         status: status,
         message: String::from(""),
         data: json!({}),
         error: String::from(""),
      }
   }

   pub fn from_status_message(status: StatusCode, message: String) -> Answer {
      Answer {
         status: status,
         message: message,
         data: json!({}),
         error: String::from(""),
      }
   }

   pub fn from_status_message_data(status: StatusCode, message: String, data: Value) -> Answer {
      Answer {
         status: status,
         message: message,
         data: data,
         error: String::from(""),
      }
   }

   pub fn from_status_error(status: StatusCode, error: String) -> Answer {
      Answer {
         status: status,
         message: String::from(""),
         data: json!({}),
         error: error,
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
         "error": self.error
      }));

      (self.status, json).into_response()
   }
}

pub type Result = core::result::Result<Success, Error>;

pub enum Success {
   TokenCreated(String),
}

impl IntoResponse for Success {
   fn into_response(self) -> Response {
      match self {
         Self::TokenCreated(token) => Answer::from_status_message_data(
            StatusCode::CREATED,
            String::from("LOGIN SUCCESS"),
            json!(token),
         )
         .into_response(),
      }
   }
}

pub enum Error {
   LoginFail,
   DatabaseConnectionFail,
   PasswordStuff,
   TokenStuff,
}

impl IntoResponse for Error {
   fn into_response(self) -> Response {
      match self {
         Self::LoginFail => Answer::from_status(StatusCode::FORBIDDEN).into_response(),
         // TODO: Improve this type of syntax formatting.
         Self::DatabaseConnectionFail | Self::PasswordStuff | Self::TokenStuff => {
            Answer::from_status(StatusCode::INTERNAL_SERVER_ERROR).into_response()
         },
      }
   }
}
