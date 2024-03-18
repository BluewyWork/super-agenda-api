use axum::http::StatusCode;
use mongodb::{
   bson::doc,
   error::{ErrorKind, WriteFailure},
};
use serde_json::json;

use crate::{
   custom::extractor::Json,
   database::mongodb_connection,
   models::{
      api::Answer,
      payload::{LoginPayload, RegisterPayload},
   },
   schemas,
   utils::{
      jwt::create_token,
      security::{hash_password, verify_password},
   },
};

pub async fn login(Json(payload): Json<LoginPayload>) -> Answer {
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

   let user_query = users_collection
      .find_one(doc! { "email": &payload.email }, None)
      .await;

   let user = match user_query {
      Ok(Some(user)) => user,
      Ok(None) => {
         return Answer {
            json: "Invalid Credentials".into(),
            status: StatusCode::UNAUTHORIZED,
            ok: false,
         };
      },
      Err(_) => {
         return Answer {
            json: "Something went wrong...".into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            ok: false,
         }
      },
   };

   if let Ok(bool) = verify_password(payload.password.to_string(), &user.password) {
      if !bool {
         return Answer {
            json: "Invalid Credentials".into(),
            status: StatusCode::UNAUTHORIZED,
            ok: false,
         };
      }
   } else {
      return Answer {
         json: "Something went wrong...".into(),
         status: StatusCode::INTERNAL_SERVER_ERROR,
         ok: false,
      };
   }

   let token = match create_token(user.username, user.email) {
      Ok(token) => token,
      Err(_) => {
         return Answer {
            json: "Unable to create token...".into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            ok: false,
         }
      },
   };

   Answer {
      json: json! ({ "token": token }),
      status: StatusCode::OK,
      ok: true,
   }
}

pub async fn register(Json(payload): Json<RegisterPayload>) -> Answer {
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

   let hashed_password = match hash_password(payload.password.to_string()) {
      Ok(password) => password,
      Err(_) => {
         return Answer {
            json: "Something went wrong...".into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            ok: false,
         }
      },
   };

   if let Err(err) = users_collection
      .insert_one(
         schemas::User {
            username: payload.username.to_string(),
            email: payload.email.to_string(),
            password: hashed_password,
         },
         None,
      )
      .await
   {
      // Specifically searchs for a duplicate key error
      // and instead of parsing the error message and extracting
      // the field which value is being duplicated
      // we can infer that the culprit is the 'email' field
      // since it is the only unique key this function handles.
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
