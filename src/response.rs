use axum::{
   http::StatusCode,
   response::{IntoResponse, Response},
   Json,
};
use serde::Serialize;
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

#[derive(Clone)]
pub enum Success {
   TokenCreated(Value),
   UserCreated,
}

impl IntoResponse for Success {
   fn into_response(self) -> Response {
      let mut response = StatusCode::OK.into_response();

      response.extensions_mut().insert(self);

      response
   }
}

impl Success {
   pub fn client_status_and_success(&self) -> (StatusCode, ClientSuccess) {
      match self {
         Self::TokenCreated(token) => (StatusCode::CREATED, ClientSuccess::Token(token.to_owned())),
         _ => (StatusCode::OK, ClientSuccess::UserCreated),
      }
   }
}

#[serde(tag = "message", content = "data")]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
pub enum ClientSuccess {
   UserCreated,
   Token(Value),
}

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
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
   InvalidUsername,
   InvalidPassword,
   InvalidEmail,
   TestError(String),
}

impl IntoResponse for Error {
   fn into_response(self) -> Response {
      let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

      response.extensions_mut().insert(self);

      response
   }
}

impl Error {
   pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
      match self {
         Self::InvalidCredentials
         | Self::UserNotFound
         | Self::UsernameOrEmailNotFound
         | Self::PasswordStuff
         | Self::TokenStuff => (StatusCode::FORBIDDEN, ClientError::InvalidCredentials),
         Self::EmailAlreadyTaken => (StatusCode::CONFLICT, ClientError::EmailAlreadyTaken),
         Self::UsernameAlreadyTaken => (StatusCode::CONFLICT, ClientError::UsernameAlreadyTaken),
         Self::DatabaseStuff | Self::DatabaseConnectionFail => {
            (StatusCode::INTERNAL_SERVER_ERROR, ClientError::ServerError)
         },
         Self::InvalidEmail => (StatusCode::UNPROCESSABLE_ENTITY, ClientError::InvalidEmail),
         Self::InvalidUsername => (
            StatusCode::UNPROCESSABLE_ENTITY,
            ClientError::InvalidUsername,
         ),
         Self::InvalidPassword => (
            StatusCode::UNPROCESSABLE_ENTITY,
            ClientError::InvalidPassword,
         ),
         Self::TestError(string) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ClientError::TestError(string.clone()),
         ),
      }
   }
}

#[derive(Debug, strum_macros::AsRefStr)]
pub enum ClientError {
   InvalidCredentials,
   EmailAlreadyTaken,
   UsernameAlreadyTaken,
   InvalidUsername,
   InvalidEmail,
   InvalidPassword,
   ServerError,
   TestError(String),
}
