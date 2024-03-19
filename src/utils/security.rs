use bcrypt;

use crate::utils::log::plog;

pub fn hash_password(password: String) -> Result<String, ()> {
   bcrypt::hash(password, 10).map_err(|_| {
      plog(
         "unable to hash password".to_string(),
         "security".to_string(),
         true,
      );
      ()
   })
}

pub fn verify_password(password: String, hash: &str) -> Result<bool, ()> {
   bcrypt::verify(password, hash).map_err(|_| {
      plog(
         "unable to verify hashed password".to_string(),
         "security".to_string(),
         true,
      );
      ()
   })
}
