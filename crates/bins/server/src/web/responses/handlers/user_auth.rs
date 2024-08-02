use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use lib_database::models::tables::user::User;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
   web::{
      custom::{extractors::Json, response::ApiResponse},
      error::{Error, Result},
      utils::{
         password::{hash_password, matches},
         token::create_token,
      },
   },
   AppState,
};

#[derive(Serialize, Deserialize)]
pub struct RegisterPayload {
   username: String,
   password: String,
}

pub async fn register(
   State(app_state): State<Arc<AppState>>,
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
      id: ObjectId::new(),
      username,
      hashed_password,
   };

   if (app_state
      .user_table
      .find_user_from_username(&user.username)
      .await)
      .is_ok()
   {
      return Err(Error::UsernameIsTaken);
   };

   let user_id_copy = user.id;

   app_state.user_table.create_user(user).await?;

   // always initialize user_data
   // and avoid other considerations if not
   app_state
      .user_data_table
      .initialize_userdata(user_id_copy)
      .await?;

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
   State(app_state): State<Arc<AppState>>,
   Json(login_payload): Json<LoginPayload>,
) -> Result<ApiResponse> {
   let LoginPayload {
      username,
      password: password_clear,
   } = login_payload;

   let user = app_state
      .user_table
      .find_user_from_username(&username)
      .await?;

   let is_same = matches(password_clear, &user.hashed_password)?;

   if !is_same {
      return Err(Error::PasswordDoesNotMatch);
   }

   let token = create_token(user.id)?;

   Ok(ApiResponse {
      status_code: StatusCode::CREATED,
      message: None,
      data: Some(json!({"token": token})),
   })
}
