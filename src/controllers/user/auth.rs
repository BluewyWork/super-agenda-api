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

   let find_one_user = if let Some(email) = payload.email {
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

   let user = match find_one_user {
      Ok(Some(user)) => user,
      Ok(None) => return Err(Error::MongoDBUserNotFound),
      Err(_) => return Err(Error::MongoDBFail),
   };

   let password_matched =
      matches(payload.password.to_string(), &user.hashed_password)?;

   if !password_matched {
      return Err(Error::PasswordIsWrong);
   }

   let token = new_token(user.username)?;

   Ok(Success::Login(json!({"token": token})))
}

pub async fn register(Json(payload): Json<RegisterPayload>) -> Result {
   let mongodb = database().await?;

   let users_collection = mongodb.collection::<User>("users");

   // Check for duplicate username.
   if let Ok(Some(_)) = users_collection
      .find_one(doc! {"username": &payload.username}, None)
      .await
   {
      return Err(Error::MongoDBDuplicateUsername);
   }

   // Check for duplicate email.
   if let Ok(Some(_)) = users_collection
      .find_one(doc! {"email": &payload.email}, None)
      .await
   {
      return Err(Error::MongoDBDuplicateEmail);
   }

   let hashed_password = hash_password(payload.password.to_string())?;

   let user = payload.to_user(hashed_password)?;

   if let Err(_) = users_collection.insert_one(user, None).await {
      return Err(Error::MongoDBInsertError);
   }

   Ok(Success::Register)
}
