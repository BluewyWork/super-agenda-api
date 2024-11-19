use bcrypt;

use crate::error::AppResult;

pub fn hash_password(password: &str) -> AppResult<String> {
   Ok(bcrypt::hash(password, 10)?)
}

pub fn matches(password: String, hashed_password: &str) -> AppResult<bool> {
   Ok(bcrypt::verify(password, hashed_password)?)
}
