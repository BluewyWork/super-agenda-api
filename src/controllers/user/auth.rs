use mongodb::{
   bson::doc,
   error::{ErrorKind, WriteFailure},
};

use crate::{
   models::{
      payload::{LoginPayload, RegisterPayload},
      schemas,
   },
   response,
   utils::database::mongodb_connection,
   utils::extractor::Json,
   utils::{
      jwt::create_token,
      security::{hash_password, verify_password},
   },
};

pub async fn login(Json(payload): Json<LoginPayload>) -> response::Result {
   let mongodb = match mongodb_connection().await {
      Ok(client) => client,
      Err(_) => return Err(response::Error::DatabaseConnectionFail),
   };

   let users_collection = mongodb.collection::<schemas::User>("users");

   let user_query = users_collection
      .find_one(doc! { "email": &payload.email }, None)
      .await;

   let user = match user_query {
      Ok(Some(user)) => user,
      Ok(None) => return Err(response::Error::LoginFail),
      Err(_) => return Err(response::Error::DatabaseConnectionFail),
   };

   if let Ok(bool) = verify_password(payload.password.to_string(), &user.password) {
      if !bool {
         return Err(response::Error::LoginFail);
      }
   } else {
      return Err(response::Error::PasswordStuff);
   }

   let token = match create_token(user.username, user.email) {
      Ok(token) => token,
      Err(_) => return Err(response::Error::TokenStuff),
   };

   Ok(response::Success::TokenCreated(token))
}

pub async fn register(Json(payload): Json<RegisterPayload>) -> response::Result {
   let mongodb = match mongodb_connection().await {
      Ok(client) => client,
      Err(_) => {
         return Err(response::Error::DatabaseConnectionFail);
      },
   };

   let users_collection = mongodb.collection::<schemas::User>("users");

   let hashed_password = match hash_password(payload.password.to_string()) {
      Ok(password) => password,
      Err(_) => {
         return Err(response::Error::PasswordStuff);
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
               return Err(response::Error::EmailAlreadyInUse);
            }
         }
      }

      return Err(response::Error::DatabaseConnectionFail);
   }

   Ok(response::Success::UserCreated)
}
