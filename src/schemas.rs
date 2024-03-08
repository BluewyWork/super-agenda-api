use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
   pub username: String,
   pub email: String,
   pub password: String,
}
