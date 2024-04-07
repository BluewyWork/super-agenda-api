use serde::Deserialize;

use super::schemas::{Phone, User};
use crate::response::error::Error;

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
   pub fn to_user(self, hashed_password: String) -> Result<User, Error> {
      User::from(
         self.display_name,
         self.username,
         hashed_password,
         self.recovery_email,
         self.email,
         self.phone,
      )
   }
}
