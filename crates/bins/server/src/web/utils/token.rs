use chrono::{TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utils::constants::JWT_SECRET;

use crate::error::{AppError, AppResult};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
   pub exp: usize,
   pub user_id: ObjectId,
}

pub fn create_token(user_id: ObjectId) -> AppResult<String> {
   let expiration_time = Utc::now()
      + match TimeDelta::try_days(30 * 6) {
         Some(time_delta) => time_delta,
         None => {
            return Err(AppError::TokenDaysOverflow);
         },
      };
   let exp_unix_timestamp = expiration_time.timestamp() as usize;

   let claims = Claims {
      user_id,
      exp: exp_unix_timestamp,
   };

   let token = encode(
      &Header::default(),
      &claims,
      &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
   )?;

   Ok(token)
}

pub fn decrypt_token(token: String) -> AppResult<Claims> {
   let claims = decode::<Claims>(
      &token,
      &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
      &Validation::default(),
   )?
   .claims;

   Ok(claims)
}
