use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::constants::JWT_SECRET;

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
   username: String,
   email: String,
}

pub fn create_token(username: String, email: String) -> Result<String, ()> {
   let token = encode(
      &Header::default(),
      &Claims { username, email },
      &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
   )
   .map_err(|_| ());

   token
}

pub fn verify_token(token: &str) -> Result<(), ()> {
   let _ = decode::<Claims>(
      &token,
      &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
      &Validation::default(),
   )
   .map_err(|_| ());

   Ok(())
}
