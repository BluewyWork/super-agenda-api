use axum::{
   http::StatusCode,
   response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::Value;

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

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "data")]
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
}
