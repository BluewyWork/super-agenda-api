use axum::{
   http::StatusCode,
   response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::Value;

#[derive(Clone)]
pub enum Success {
   Login(Value),
   Register,
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
         Self::Login(token) => (
            StatusCode::CREATED,
            ClientSuccess::LOGGED_IN(token.to_owned()),
         ),
         Self::Register => (StatusCode::OK, ClientSuccess::REGISTERED),
      }
   }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "data")]
pub enum ClientSuccess {
   REGISTERED,
   LOGGED_IN(Value),
}
