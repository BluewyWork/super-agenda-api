use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
   username: String,
   email: String,
}

pub fn create_token(username: String, email: String) -> Result<String, ()> {
   let jwt_secret_name = "JWT_SECRET";
   let jwt_secret_value = match std::env::var(jwt_secret_name) {
      Ok(env) => env,
      Err(_) => {
         println!("env: {} not found", jwt_secret_name);
         return Err(());
      },
   };

   let token = encode(
      &Header::default(),
      &Claims { username, email },
      &EncodingKey::from_secret(jwt_secret_value.as_bytes()),
   )
   .map_err(|_| ());

   token
}

pub fn verify_token(claims: Claims, token: &str) -> Result<(), ()> {
   let jwt_secret_name = "JWT_SECRET";
   let jwt_secret_value = match std::env::var(jwt_secret_name) {
      Ok(env) => env,
      Err(_) => {
         println!("env: {} not found", jwt_secret_name);
         return Err(());
      },
   };

   let token = decode::<Claims>(
      &token,
      &DecodingKey::from_secret(jwt_secret_value.as_bytes()),
      &Validation::default(),
   );

   Ok(())
}
