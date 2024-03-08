use axum::{http::StatusCode, Json};
use mongodb::error::{ErrorKind, WriteFailure};

use crate::{
   database::mongodb_connection,
   models::{
      api::Answer,
      auth::{LoginPayload, RegisterPayload},
   },
   schemas,
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

pub async fn register(payload: Json<RegisterPayload>) -> Answer {
   let mongodb = match mongodb_connection().await {
      Ok(client) => client,
      Err(_) => {
         return Answer {
            json: "Something went wrong.".into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            ok: false,
         }
      },
   };

   let users_collection = mongodb.collection::<schemas::User>("users");

   if let Err(err) = users_collection
      .insert_one(
         schemas::User {
            username: payload.username.to_string(),
            email: payload.email.to_string(),
            password: payload.password.to_string(),
         },
         None,
      )
      .await
   {
      if let ErrorKind::Write(write_failure) = *err.kind {
         if let WriteFailure::WriteError(write_error) = write_failure {
            if write_error.code == 11000 {
               return Answer {
                  json: "Email already in use.".into(),
                  status: StatusCode::CONFLICT,
                  ok: false,
               };
            }
         }
      }

      return Answer {
         json: "Something went wrong.".into(),
         status: StatusCode::INTERNAL_SERVER_ERROR,
         ok: false,
      };
   }

   Answer {
      json: "User Registered Sucessfully".into(),
      status: StatusCode::OK,
      ok: true,
   }
}
