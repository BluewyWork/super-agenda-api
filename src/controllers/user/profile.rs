use mongodb::bson::doc;
use serde_json::json;

use crate::{
   models::{payload::UserPayload, schemas::User},
   response::{self, error::Error, success::Success, Result},
   utils::{
      extractor::Json, jwt::Claims, mongo::database
   },
};

pub async fn me(claims: Claims) -> Result {
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

         Ok(Success::Me(json!({"user": serialized_user_payload})))
      },
      Ok(None) => Err(Error::MongoDBUserNotFound),
      Err(_) => Err(Error::MongoDBFail),
   }
}

pub async fn update(claims: Claims, Json(user_payload): Json<UserPayload>) -> Result {

   todo!()
}
