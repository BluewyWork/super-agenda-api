use axum::{
   http::StatusCode,
   response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::Value;

#[derive(Clone)]
pub enum Success {
   LoginResult(Value),
   RegisterResult,
   ShowUserProfileResult(Value),
   UpdateUserProfileResult,
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
         Self::RegisterResult => (StatusCode::OK, ClientSuccess::REGISTERED),
         Self::UpdateUserProfileResult => (StatusCode::OK, ClientSuccess::PROFILE_UPDATED),

         Self::ShowUserProfileResult(json) => {
            (StatusCode::OK, ClientSuccess::PROFILE_SHOWED(json.clone()))
         },

         Self::LoginResult(token) => (
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
   PROFILE_SHOWED(Value),
   PROFILE_UPDATED,
}
