use axum::{extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
   web::{
      custom::{extractors::Json, response::ApiResponse},
      error::{Error, Result},
      utils::{password::matches, token::create_token},
   },
   ApiState,
};

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
   username: String,
   password: String,
}

pub async fn login(
   State(api_state): State<ApiState>,
   Json(login_payload): Json<LoginPayload>,
) -> Result<ApiResponse> {
   let LoginPayload {
      username,
      password: password_clear,
   } = login_payload;

   let admin = api_state
      .admin_table
      .find_admin_from_username(&username)
      .await?;

   let is_same = matches(password_clear, &admin.hashed_password)?;

   if !is_same {
      return Err(Error::PasswordDoesNotMatch);
   }

   let token = create_token(admin._id)?;

   Ok(ApiResponse {
      status_code: StatusCode::CREATED,
      message: None,
      data: Some(json!({"token": token})),
   })
}
