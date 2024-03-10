use bcrypt;

pub fn hash_password(password: String) -> Result<String, ()> {
   bcrypt::hash(password, 10).map_err(|_| ())
}

pub fn verify_password(password: String, hash: &str) -> Result<bool, ()> {
   bcrypt::verify(password, hash).map_err(|_| ())
}
