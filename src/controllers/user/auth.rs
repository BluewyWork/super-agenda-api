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
      jwt::new_token,
      mongo::database,
      password_stuff::{hash_password, is_valid_password},
   },
};

pub async fn login(Json(payload): Json<LoginPayload>) -> Result {
   let mongodb = database().await?;

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
      Ok(None) => return Err(Error::MongoDBUserNotFound),
      Err(_) => return Err(Error::MongoDBError),
   };

   let password_ok = is_valid_password(payload.password.to_string(), &user.hashed_password)?;

   if !password_ok {
      return Err(Error::PasswordNotValid);
   }

   let token = new_token(user.username)?;

   Ok(Success::Login(json!({"token": token})))
}

pub async fn register(Json(payload): Json<RegisterPayload>) -> Result {
   let mongodb = database().await?;

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

   let hashed_password = hash_password(payload.password.to_string())?;

   let user = payload.to_user(hashed_password)?;

   if let Err(_) = users_collection.insert_one(user, None).await {
      return Err(Error::MongoDBInsertError);
   }

   Ok(Success::Register)
}
