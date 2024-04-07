use axum::{
   http::StatusCode,
   response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
pub enum Error {
   MongoDBUserNotFound,
   UsernameOrEmailNotFound,
   EmailAlreadyTaken,
   UsernameAlreadyTaken,
   PasswordNotValid,
   PasswordHashError,
   PasswordVerificationError,
   TokenNotFound,
   TokenNotVerified,
   TokenNotCreated,
   MongoDBParserError,
   MongoDBNoClient,
   MongoDBInsertError,
   MongoDBError,
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
         Self::MongoDBUserNotFound | Self::UsernameOrEmailNotFound => {
            (StatusCode::FORBIDDEN, ClientError::INVALID_CREDENTIALS)
         },
         Self::EmailAlreadyTaken => (StatusCode::CONFLICT, ClientError::EMAIL_ALREADY_TAKEN),
         Self::UsernameAlreadyTaken => (StatusCode::CONFLICT, ClientError::USERNAME_ALREADY_TAKEN),
         Self::InvalidEmail => (StatusCode::UNPROCESSABLE_ENTITY, ClientError::INVALID_EMAIL),
         Self::InvalidUsername => (
            StatusCode::UNPROCESSABLE_ENTITY,
            ClientError::INVALID_USERNAME,
         ),
         Self::InvalidPassword => (
            StatusCode::UNPROCESSABLE_ENTITY,
            ClientError::INVALID_PASSWORD,
         ),
         _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVER_ERROR),
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
