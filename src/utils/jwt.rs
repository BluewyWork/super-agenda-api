use chrono::{TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{
   response::error::Error,
   utils::{constants::JWT_SECRET, log::plog},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
   pub username: String,
   pub exp: usize,
}

pub fn new_token(username: String) -> Result<String, Error> {
   let expiration_time = Utc::now()
      + match TimeDelta::try_days(30 * 6) {
         Some(time_delta) => time_delta,
         None => {
            plog(
               "number size is bigger than supported".to_string(),
               "jwt".to_string(),
               true,
            );
            return Err(Error::NumberOverflow);
         },
      };
   let exp_unix_timestamp = expiration_time.timestamp() as usize;

   let claims = Claims {
      username,
      exp: exp_unix_timestamp,
   };

   match encode(
      &Header::default(),
      &claims,
      &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
   ) {
      Ok(token) => Ok(token),
      Err(err) => {
         plog(format!("{:?}", err.kind()), "jwt".to_string(), true);
         Err(Error::TokenCreation)
      },
   }
}

pub fn verify_token(token: String) -> Result<Claims, Error> {
   match decode::<Claims>(
      &token,
      &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
      &Validation::default(),
   ) {
      Ok(token_data) => Ok(token_data.claims),
      Err(err) => {
         plog(format!("{:?}", err.kind()), "jwt".to_string(), true);
         Err(Error::TokenInvalid)
      },
   }
}
