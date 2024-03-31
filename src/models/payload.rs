use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
   pub email: String,
   pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
   pub username: String,
   pub password: String,
   pub display_name: String,
}
