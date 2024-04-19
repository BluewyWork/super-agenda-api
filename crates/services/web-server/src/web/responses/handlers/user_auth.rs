use axum::{extract::State, http::StatusCode};
use lib_database::models::tables::user::{User, UserTable};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::web::{
   custom::{extractors::Json, response::ApiResponse},
   error::{Error, Result},
   utils::{
      password::{hash_password, matches},
      token::create_token,
   },
};

#[derive(Serialize, Deserialize)]
pub struct RegisterPayload {
   username: String,
   password: String,
}

pub async fn register(
   State(user_table): State<UserTable>,
   Json(register_payload): Json<RegisterPayload>,
) -> Result<ApiResponse> {
   let RegisterPayload {
      username,
      password: password_clear,
   } = register_payload;

   if username.len() < 5 {
      return Err(Error::UsernameTooShort);
   }

   if password_clear.len() < 5 {
      return Err(Error::PasswordTooShort);
   }

   let hashed_password = hash_password(&password_clear)?;

   let user = User {
      _id: ObjectId::new(),
      username,
      hashed_password,
   };

   user_table.create_user(user).await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
   username: String,
   password: String,
}

pub async fn login(
   State(user_table): State<UserTable>,
   Json(login_payload): Json<LoginPayload>,
) -> Result<ApiResponse> {
   let LoginPayload {
      username,
      password: password_clear,
   } = login_payload;

   let user = user_table.find_user_from_username(&username).await?;

   let is_same = matches(password_clear, &user.hashed_password)?;

   if !is_same {
      return Err(Error::PasswordDoesNotMatch);
   }

   let token = create_token(user._id)?;

   Ok(ApiResponse {
      status_code: StatusCode::CREATED,
      message: None,
      data: Some(json!({"token": token})),
   })
}
