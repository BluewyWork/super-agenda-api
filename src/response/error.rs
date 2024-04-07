use axum::{
   http::StatusCode,
   response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
pub enum Error {
   InvalidCredentials,
   UserNotFound,
   UsernameOrEmailNotFound,
   EmailAlreadyTaken,
   UsernameAlreadyTaken,
   PasswordStuff,
   PasswordHashError,
   PasswordVerificationError,
   TokenStuff,
   TokenNotVerified,
   TokenNotCreated,
   TokenNotFound,
   MongoDBStuff,
   MongoDBParserError,
   MongoDBNoClient,
   InvalidUsername,
   InvalidPassword,
   InvalidEmail,
   NumberOverflow,
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
         | Self::TokenStuff => (StatusCode::FORBIDDEN, ClientError::INVALID_CREDENTIALS),
         Self::EmailAlreadyTaken => (StatusCode::CONFLICT, ClientError::EMAIL_ALREADY_TAKEN),
         Self::UsernameAlreadyTaken => (StatusCode::CONFLICT, ClientError::USERNAME_ALREADY_TAKEN),
         Self::MongoDBStuff => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVER_ERROR),
         Self::InvalidEmail => (StatusCode::UNPROCESSABLE_ENTITY, ClientError::INVALID_EMAIL),
         Self::InvalidUsername => (
            StatusCode::UNPROCESSABLE_ENTITY,
            ClientError::INVALID_USERNAME,
         ),
         Self::InvalidPassword => (
            StatusCode::UNPROCESSABLE_ENTITY,
            ClientError::INVALID_PASSWORD,
         ),
      }
   }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
   INVALID_CREDENTIALS,
   EMAIL_ALREADY_TAKEN,
   USERNAME_ALREADY_TAKEN,
   INVALID_USERNAME,
   INVALID_EMAIL,
   INVALID_PASSWORD,
   SERVER_ERROR,
}
