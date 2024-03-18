use axum::{
   extract::rejection::JsonRejection, extract::FromRequest, http::StatusCode,
   response::IntoResponse,
};
use serde::Serialize;

use crate::models::api::Answer;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(Answer))]
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
   fn into_response(self) -> axum::response::Response {
      let Self(value) = self;
      axum::Json(value).into_response()
   }
}

impl From<JsonRejection> for Answer {
   fn from(rejection: JsonRejection) -> Self {
      Self {
         json: rejection.body_text().into(),
         status: rejection.status(),
         ok: false,
      }
   }
}
