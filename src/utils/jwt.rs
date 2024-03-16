use chrono::{TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::constants::JWT_SECRET;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
   pub username: String,
   pub email: String,
   pub exp: usize,
}

pub fn create_token(username: String, email: String) -> Result<String, ()> {
   let expiration_time = Utc::now()
      + match TimeDelta::try_days(30 * 6) {
         Some(time_delta) => time_delta,
         None => {
            println!("api: time unit overflow");
            return Err(());
         },
      };
   let exp_unix_timestamp = expiration_time.timestamp() as usize;

   let claims = Claims {
      username,
      email,
      exp: exp_unix_timestamp,
   };

   match encode(
      &Header::default(),
      &claims,
      &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
   ) {
      Ok(token) => Ok(token),
      Err(err) => {
         println!("api: unable to create token -> {:?}", err);
         Err(())
      },
   }
}

pub fn verify_token(token: String) -> Result<Claims, ()> {
   match decode::<Claims>(
      &token,
      &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
      &Validation::default(),
   ) {
      Ok(token_data) => Ok(token_data.claims),
      Err(err) => {
         println!("api: unable to verify token -> {:?}", err);
         Err(())
      },
   }
}
