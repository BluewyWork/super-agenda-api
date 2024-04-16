use chrono::{TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lib_utils::constants::JWT_SECRET;
use serde::{Deserialize, Serialize};

use crate::web::error::{Error, Result};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
   pub username: String,
   pub exp: usize,
}

pub fn create_token(username: String) -> Result<String> {
   let expiration_time = Utc::now()
      + match TimeDelta::try_days(30 * 6) {
         Some(time_delta) => time_delta,
         None => {
            return Err(Error::TokenDaysOverflow);
         },
      };
   let exp_unix_timestamp = expiration_time.timestamp() as usize;

   let claims = Claims {
      username,
      exp: exp_unix_timestamp,
   };

   let token = encode(
      &Header::default(),
      &claims,
      &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
   )?;

   Ok(token)
}

pub fn decrypt_token(token: String) -> Result<Claims> {
   let claims = decode::<Claims>(
      &token,
      &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
      &Validation::default(),
   )?
   .claims;

   Ok(claims)
}
