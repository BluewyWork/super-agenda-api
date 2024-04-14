use axum::{
   http::StatusCode,
   response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::Value;

#[derive(Clone)]
pub enum Success {
   AuthLogin(Value),
   AuthRegister,
   UserShow(Value),
   UserUpdation,
   UserDeletion,
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
         Self::AuthRegister => (StatusCode::OK, ClientSuccess::REGISTERED),
         Self::UserUpdation => (StatusCode::OK, ClientSuccess::USER_UPDATED),
         Self::UserDeletion => (StatusCode::OK, ClientSuccess::USER_DELETED),

         Self::UserShow(json) => (StatusCode::OK, ClientSuccess::USER_SHOWED(json.clone())),

         Self::AuthLogin(token) => (
            StatusCode::CREATED,
            ClientSuccess::LOGGED_IN(token.to_owned()),
         ),
      }
   }
}

#[allow(warnings)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "data")]
pub enum ClientSuccess {
   REGISTERED,
   LOGGED_IN(Value),
   USER_SHOWED(Value),
   USER_UPDATED,
   USER_DELETED,
}
