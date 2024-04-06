use mongodb::bson::doc;
use serde_json::json;

use crate::{
   models::{
      payload::{LoginPayload, RegisterPayload},
      schemas,
   },
   response::{error::Error, success::Success, Result},
   utils::{
      extractor::Json,
      jwt::create_token,
      mongodb::database,
      password_stuff::{hash_password, verify_password},
   },
};

pub async fn login(Json(payload): Json<LoginPayload>) -> Result {
   let mongodb = match database().await {
      Ok(client) => client,
      Err(_) => return Err(Error::MongoDBStuff),
   };

   let users_collection = mongodb.collection::<schemas::User>("users");

   // Use either email or username to find user.
   let user_query = if let Some(email) = payload.email {
      users_collection
         .find_one(doc! { "email": email }, None)
         .await
   } else if let Some(username) = payload.username {
      users_collection
         .find_one(doc! { "username": username }, None)
         .await
   } else {
      return Err(Error::UsernameOrEmailNotFound);
   };

   let user = match user_query {
      Ok(Some(user)) => user,
      Ok(None) => return Err(Error::UserNotFound),
      Err(_) => return Err(Error::MongoDBStuff),
   };

   if let Ok(bool) = verify_password(payload.password.to_string(), &user.hashed_password) {
      if !bool {
         return Err(Error::InvalidCredentials);
      }
   } else {
      return Err(Error::PasswordStuff);
   }

   let token = match create_token(user.username) {
      Ok(token) => token,
      Err(_) => return Err(Error::TokenStuff),
   };

   Ok(Success::Token(json!({"token": token})))
}

pub async fn register(Json(payload): Json<RegisterPayload>) -> Result {
   let mongodb = match database().await {
      Ok(client) => client,
      Err(_) => {
         return Err(Error::MongoDBStuff);
      },
   };

   let users_collection = mongodb.collection::<schemas::User>("users");

   // Check for duplicate username.
   if let Ok(Some(_)) = users_collection
      .find_one(doc! {"username": &payload.username}, None)
      .await
   {
      return Err(Error::UsernameAlreadyTaken);
   }

   // Check for duplicate email.
   match &payload.email {
      Some(email) => {
         if let Ok(Some(_)) = users_collection.find_one(doc! {"email": email}, None).await {
            return Err(Error::EmailAlreadyTaken);
         }
      },
      None => {},
   }

   let hashed_password = match hash_password(payload.password.to_string()) {
      Ok(password) => password,
      Err(_) => {
         return Err(Error::PasswordStuff);
      },
   };

   let user = match payload.to_user(hashed_password) {
      Ok(user) => user,
      Err(err) => return Err(err),
   };

   if let Err(_) = users_collection.insert_one(user, None).await {
      return Err(Error::MongoDBStuff);
   }

   Ok(Success::User)
}
