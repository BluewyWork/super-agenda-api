use mongodb::bson::doc;
use serde_json::json;

use crate::{
   models::{payload::UserPayload, schemas::User},
   response::{error::Error, success::Success, Result},
   utils::{extractor::Json, jwt::Claims, mongo::database},
};

pub async fn show(claims: Claims) -> Result {
   let mongodb = database().await?;
   let users_collection = mongodb.collection::<User>("users");

   let find_user = users_collection
      .find_one(doc! {"username": claims.username}, None)
      .await;

   match find_user {
      Ok(Some(user)) => {
         let user_payload = UserPayload {
            username: user.username,
            display_name: user.display_name,
            email: user.email,
            recovery_email: user.recovery_email,
            phone: user.phone,
         };

         let serialized_user_payload =
            serde_json::to_value(user_payload).map_err(|_| Error::JsonSerializationFail)?;

         Ok(Success::ShowUserProfileResult(
            json!({"user": serialized_user_payload}),
         ))
      },
      Ok(None) => Err(Error::MongoDBUserNotFound),
      Err(_) => Err(Error::MongoDBFail),
   }
}

pub async fn update(claims: Claims, Json(user_payload): Json<UserPayload>) -> Result {
   let mongodb = database().await?;
   let users_collection = mongodb.collection::<User>("users");

   let find_user = users_collection
      .find_one(doc! {"username": claims.username}, None)
      .await;

   let username = match find_user {
      Ok(Some(user)) => user.username,
      Ok(None) => return Err(Error::MongoDBUserNotFound),
      Err(_) => return Err(Error::MongoDBFail),
   };

   let update_query = doc! {
      "$set": doc! {
         "username": &user_payload.username,
         "display_name": user_payload.display_name,
         "email": user_payload.email,
         "recovery_email": user_payload.recovery_email,
         "phone": user_payload.phone
      }
   };

   match users_collection
      .update_one(doc! {"username": username}, update_query, None)
      .await
   {
      Ok(_) => Ok(Success::UpdateUserProfileResult),
      Err(_) => Err(Error::UpdateUserProfileFail),
   }
}
