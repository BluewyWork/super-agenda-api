use axum::{http::StatusCode, Json};

use crate::{
   database::mongodb_connection,
   models::{
      api::Answer,
      auth::{LoginPayload, RegisterPaylaod},
   },
};

pub async fn login(payload: Json<LoginPayload>) -> Answer {
   if payload.username != "demo" || payload.password != "demo" {
      return Answer {
         json: "Invalid Credentials".into(),
         status: StatusCode::UNAUTHORIZED,
         ok: true,
      };
   }

   Answer {
      json: "Login Successful".into(),
      status: StatusCode::OK,
      ok: true,
   }
}

pub async fn register(payload: RegisterPaylaod) -> Answer {
   Answer {
      json: "User Registered Sucessfully".into(),
      status: StatusCode::OK,
      ok: true,
   }
}
