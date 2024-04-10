use axum::{
   http::StatusCode,
   response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
pub enum Error {
   PayloadUsernameOrEmailNotFound,
   PasswordIsWrong,
   PasswordHashingFail,
   PasswordVerificationFail,
   TokenNotFound,
   TokenInvalid,
   TokenCreationFail,
   MongoDBDuplicateEmail,
   MongoDBDuplicateUsername,
   MongoDBUserNotFound,
   MongoDBParserFail,
   MongoDBNoClient,
   MongoDBInsertFail,
   MongoDBFail,
   InvalidUsername,
   InvalidPassword,
   InvalidEmail,
   NumberOverflow,
   ClaimsNotFound,
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
         Self::MongoDBDuplicateEmail => (StatusCode::CONFLICT, ClientError::UNAVAILABLE_EMAIL),
         Self::InvalidEmail => (StatusCode::UNPROCESSABLE_ENTITY, ClientError::INVALID_EMAIL),
         Self::TokenNotFound => (StatusCode::BAD_REQUEST, ClientError::MISSING_TOKEN),
         Self::TokenInvalid => (StatusCode::FORBIDDEN, ClientError::INVALID_TOKEN),

         Self::ClaimsNotFound => (
            StatusCode::UNAUTHORIZED,
            ClientError::AUTHORIZATION_REQUIRED,
         ),

         Self::PayloadUsernameOrEmailNotFound => {
            (StatusCode::BAD_REQUEST, ClientError::INVALID_CREDENTIALS)
         },

         Self::MongoDBDuplicateUsername => {
            (StatusCode::CONFLICT, ClientError::UNAVAILABLE_USERNAME)
         },

         Self::InvalidUsername => (
            StatusCode::UNPROCESSABLE_ENTITY,
            ClientError::INVALID_USERNAME,
         ),

         Self::InvalidPassword => (
            StatusCode::UNPROCESSABLE_ENTITY,
            ClientError::INVALID_PASSWORD,
         ),

         Self::MongoDBUserNotFound | Self::PasswordIsWrong => {
            (StatusCode::FORBIDDEN, ClientError::INVALID_CREDENTIALS)
         },

         Error::TokenCreationFail
         | Error::MongoDBParserFail
         | Error::MongoDBNoClient
         | Error::MongoDBInsertFail
         | Error::MongoDBFail
         | Self::PasswordVerificationFail
         | Self::PasswordHashingFail
         | Error::NumberOverflow => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVER_ERROR),
      }
   }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
   UNAVAILABLE_EMAIL,
   UNAVAILABLE_USERNAME,
   MISSING_TOKEN,
   INVALID_TOKEN,
   INVALID_CREDENTIALS,
   INVALID_USERNAME,
   INVALID_EMAIL,
   INVALID_PASSWORD,
   AUTHORIZATION_REQUIRED,
   SERVER_ERROR,
}
