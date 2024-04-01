use serde::Deserialize;

use super::schemas::{Phone, User};

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
   pub username: Option<String>,
   pub email: Option<String>,
   pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
   pub display_name: String,
   pub username: String,
   pub password: String,
   pub email: Option<String>,
   pub recovery_email: Option<String>,
   pub phone: Option<Phone>,
}

impl RegisterPayload {
   pub fn to_user(self, hashed_password: String) -> User {
      User {
         display_name: self.display_name,
         username: self.username,
         password: hashed_password,
         recovery_email: self.recovery_email,
         email: self.email,
         phone: self.phone,
      }
   }
}
