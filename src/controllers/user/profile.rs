use axum::{extract::Request, response::Response};

use crate::{
   models::schemas::User,
   response::{self, error::Error, success::Success},
   utils::{
      jwt::Claims,
      mongo::{self, database},
   },
};

pub async fn me(request: Request) -> response::Result {
   let claims = match request.extensions().get::<Claims>() {
      Some(claims) => claims,
      None => return Err(Error::TokenNotFound),
   };

   let mongodb = database().await?;
   //  {
   //    Ok(mongodb) => mongodb,
   //    Err(_) => return Err(Error::MongoDBStuff),
   // };

   Ok(Success::Register)
}
