use bcrypt;

use crate::{response::error::Error, utils::log::plog};

pub fn hash_password(password: String) -> Result<String, Error> {
   bcrypt::hash(password, 10).map_err(|_| {
      plog(
         "unable to hash password".to_string(),
         "security".to_string(),
         true,
      );
      Error::PasswordHashingError
   })
}

pub fn matches(password: String, hash: &str) -> Result<bool, Error> {
   bcrypt::verify(password, hash).map_err(|_| {
      plog(
         "unable to verify hashed password".to_string(),
         "security".to_string(),
         true,
      );
      Error::PasswordVerificationError
   })
}
