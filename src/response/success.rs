use axum::{
   http::StatusCode,
   response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::Value;

#[derive(Clone)]
pub enum Success {
   Token(Value),
   User,
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
         Self::Token(token) => (StatusCode::CREATED, ClientSuccess::Login(token.to_owned())),
         _ => (StatusCode::OK, ClientSuccess::Register),
      }
   }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "data")]
pub enum ClientSuccess {
   Register,
   Login(Value),
}
