use bcrypt;

pub fn hash_password(password: String) -> Result<String, ()> {
   bcrypt::hash(password, 10).map_err(|_| {
      println!("api: unable to hash password");
      ()
   })
}

pub fn verify_password(password: String, hash: &str) -> Result<bool, ()> {
   bcrypt::verify(password, hash).map_err(|_| {
      println!("api: unable to verify hashed password");
      ()
   })
}
