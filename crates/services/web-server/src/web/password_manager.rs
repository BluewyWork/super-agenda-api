use bcrypt;

use crate::web::error::Result;

pub fn hash_password(password: &str) -> Result<String> {
   Ok(bcrypt::hash(password, 10)?)
}

pub fn matches(password: String, hashed_password: &str) -> Result<bool> {
   Ok(bcrypt::verify(password, hashed_password)?)
}
