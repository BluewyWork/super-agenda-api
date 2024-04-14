use mongodb::bson::doc;
use serde_json::json;

use crate::{
   models::{
      payload::{LoginPayload, RegisterPayload},
      schemas::User,
   },
   response::{error::Error, success::Success, Result},
   utils::{
      extractor::Json,
      jwt::new_token,
      mongo::database,
      password_stuff::{hash_password, matches},
   },
};

pub async fn login(Json(payload): Json<LoginPayload>) -> Result {
   let mongodb = database().await?;
   let users_collection = mongodb.collection::<User>("users");

   let find_user = if let Some(email) = payload.email {
      users_collection
         .find_one(doc! { "email": email }, None)
         .await
   } else if let Some(username) = payload.username {
      users_collection
         .find_one(doc! { "username": username }, None)
         .await
   } else {
      return Err(Error::PayloadUsernameOrEmailNotFound);
   };

   let user = match find_user {
      Ok(Some(user)) => user,
      Ok(None) => return Err(Error::MongoDBUserNotFound),
      Err(_) => return Err(Error::MongoDBFail),
   };

   let password_matched = matches(payload.password.to_string(), &user.hashed_password)?;

   if !password_matched {
      return Err(Error::PasswordIsWrong);
   }

   let token = new_token(user.username)?;

   Ok(Success::TokenCreation(json!({"token": token})))
}

pub async fn register(Json(payload): Json<RegisterPayload>) -> Result {
   let mongodb = database().await?;
   let users_collection = mongodb.collection::<User>("users");

   // Duplicate usernames are not allowed.
   if let Ok(Some(_)) = users_collection
      .find_one(doc! {"username": &payload.username}, None)
      .await
   {
      return Err(Error::MongoDBDuplicateUsername);
   }

   // Duplicate emails are not allowed.
   if let Ok(Some(_)) = users_collection
      .find_one(doc! {"email": &payload.email}, None)
      .await
   {
      return Err(Error::MongoDBDuplicateEmail);
   }

   let hashed_password = hash_password(payload.password.to_string())?;
   let user = payload.to_user(hashed_password)?;

   if (users_collection.insert_one(user, None).await).is_err() {
      return Err(Error::MongoDBInsert);
   }

   Ok(Success::UserCreation)
}
