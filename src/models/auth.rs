use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
   pub username: String,
   pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterPaylaod {
   pub username: String,
   pub email: String,
   pub password: String,
}
