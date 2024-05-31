use axum::{
   async_trait,
   extract::{
      rejection::{JsonRejection, PathRejection},
      FromRequest, FromRequestParts,
   },
   http::request::Parts,
   response::IntoResponse,
};
use serde::Serialize;

use crate::web::{error::Error, utils::token::Claims};

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(Error))]
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
   fn into_response(self) -> axum::response::Response {
      let Self(value) = self;
      axum::Json(value).into_response()
   }
}

impl From<JsonRejection> for Error {
   fn from(_rejection: JsonRejection) -> Self {
      Self::JsonExtraction
   }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Claims {
   type Rejection = Error;

   async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
      parts
         .extensions
         .get::<Claims>()
         .cloned()
         .ok_or(Error::ClaimsNotFound)
   }
}
