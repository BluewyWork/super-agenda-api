use axum::{extract::rejection::JsonRejection, extract::FromRequest, response::IntoResponse};
use serde::Serialize;

use crate::response::Answer;

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
      Self::from_status_message(rejection.status(), rejection.body_text().into())
   }
}
