use axum::{
   extract::{rejection::JsonRejection, FromRequest},
   response::IntoResponse,
};
use serde::Serialize;

use crate::web::error::Error;

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
