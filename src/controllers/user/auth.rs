use mongodb::bson::doc;

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
      .find_one(doc! { "username": &payload.username }, None)
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

   let token = match create_token(user.username) {
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

   // Check for duplicate username.

   if let Ok(Some(_)) = users_collection
      .find_one(doc! {"username": &payload.username}, None)
      .await
   {
      return Err(response::Error::UsernameAlreadyTaken);
   }

   // Check for duplicate email.

   match &payload.email {
      Some(email) => {
         if let Ok(Some(_)) = users_collection.find_one(doc! {"email": email}, None).await {
            return Err(response::Error::EmailAlreadyTaken);
         }
      },
      None => {},
   }

   let hashed_password = match hash_password(payload.password.to_string()) {
      Ok(password) => password,
      Err(_) => {
         return Err(response::Error::PasswordStuff);
      },
   };

   if let Err(_) = users_collection
      .insert_one(payload.to_user(hashed_password), None)
      .await
   {
      return Err(response::Error::DatabaseConnectionFail);
   }

   Ok(response::Success::UserCreated)
}
