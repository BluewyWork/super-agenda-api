use std::fmt::Display;

use axum::{
   http::StatusCode,
   response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
   JsonExtraction,
   UsernameTooShort,
   PasswordTooShort,
   PasswordDoesNotMatch,
   TokenDaysOverflow,
   TokenNotFound,
   ClaimsNotFound,

   LibDatabase(lib_database::error::Error),
   Bcrypt(String),
   JsonWebToken(jsonwebtoken::errors::Error),
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
         Self::JsonExtraction => (StatusCode::BAD_REQUEST, ClientError::UNEXPECTED_BODY),
         Self::PasswordDoesNotMatch => (StatusCode::FORBIDDEN, ClientError::INVALID_CREDENTIALS),
         Self::ClaimsNotFound | Self::TokenNotFound => {
            (StatusCode::BAD_REQUEST, ClientError::INVALID_CREDENTIALS)
         },

         Self::UsernameTooShort => (
            StatusCode::UNPROCESSABLE_ENTITY,
            ClientError::USERNAME_TOO_SHORT,
         ),
         Self::PasswordTooShort => (
            StatusCode::UNPROCESSABLE_ENTITY,
            ClientError::PASSWORD_TOO_SHORT,
         ),

         Self::TokenDaysOverflow
         | Self::Bcrypt(_)
         | Self::LibDatabase(_)
         | Self::JsonWebToken(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ClientError::SERVICE_ERROR,
         ),
      }
   }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
   USERNAME_TOO_SHORT,
   PASSWORD_TOO_SHORT,
   INVALID_CREDENTIALS,
   UNEXPECTED_BODY,
   SERVICE_ERROR,
}

impl Display for Error {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let message = match self {
         Self::JsonExtraction => String::from("json signature does not match"),
         Self::PasswordTooShort => String::from("password too short. 5 characters min."),
         Self::UsernameTooShort => String::from("username too short. 5 characters min"),
         Self::PasswordDoesNotMatch => String::from("password does not match"),
         Self::TokenDaysOverflow => String::from("unable to create token due to size overflow"),
         Self::TokenNotFound => String::from("no token found"),
         Self::ClaimsNotFound => String::from("claims not found"),

         Self::Bcrypt(string) => string.to_string(),
         Self::LibDatabase(err) => err.to_string(),
         Self::JsonWebToken(err) => err.to_string(),
      };

      write!(f, "ERROR => {message}")
   }
}

impl From<lib_database::error::Error> for Error {
   fn from(err: lib_database::error::Error) -> Self {
      Self::LibDatabase(err)
   }
}

impl From<bcrypt::BcryptError> for Error {
   fn from(err: bcrypt::BcryptError) -> Self {
      Self::Bcrypt(err.to_string())
   }
}

impl From<jsonwebtoken::errors::Error> for Error {
   fn from(err: jsonwebtoken::errors::Error) -> Self {
      Self::JsonWebToken(err)
   }
}
