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

pub type Result = core::result::Result<Success, Error>;

pub enum Success {
   TokenCreated(String),
   UserCreated,
}

impl IntoResponse for Success {
   fn into_response(self) -> Response {
      match self {
         Self::TokenCreated(token) => Answer::from_status_message_data(
            StatusCode::CREATED,
            String::from("LOGIN SUCCESS"),
            json!({ "token": token }),
         )
         .into_response(),
         Self::UserCreated => {
            Answer::from_status_message(StatusCode::CREATED, String::from("REGISTER SUCCESS"))
               .into_response()
         },
      }
   }
}

pub enum Error {
   InvalidCredentials,
   UserNotFound,
   UsernameOrEmailNotFound,
   EmailAlreadyTaken,
   UsernameAlreadyTaken,
   PasswordStuff,
   TokenStuff,
   DatabaseConnectionFail,
   DatabaseStuff,
   InvalidUsername(String),
   InvalidPassword(String),
   InvalidEmail(String),
}

impl IntoResponse for Error {
   fn into_response(self) -> Response {
      match self {
         Self::InvalidCredentials
         | Self::UserNotFound
         | Self::UsernameOrEmailNotFound
         | Self::PasswordStuff
         | Self::TokenStuff => {
            Answer::from_status_message(StatusCode::CONFLICT, String::from("INVALID CREDENTIALS"))
               .into_response()
         },
         Self::EmailAlreadyTaken => {
            Answer::from_status_message(StatusCode::CONFLICT, String::from("EMAIL ALREADY TAKEN"))
               .into_response()
         },
         Self::UsernameAlreadyTaken => Answer::from_status_message(
            StatusCode::FORBIDDEN,
            String::from("USERNAME ALREADY TAKEN"),
         )
         .into_response(),
         Self::DatabaseStuff | Self::DatabaseConnectionFail => {
            Answer::from_status(StatusCode::INTERNAL_SERVER_ERROR).into_response()
         },
         Self::InvalidEmail(msg) => {
            Answer::from_status_message(StatusCode::UNPROCESSABLE_ENTITY, msg).into_response()
         },
         Self::InvalidUsername(msg) => {
            Answer::from_status_message(StatusCode::UNPROCESSABLE_ENTITY, msg).into_response()
         },
         Self::InvalidPassword(msg) => {
            Answer::from_status_message(StatusCode::UNPROCESSABLE_ENTITY, msg).into_response()
         },
      }
   }
}
