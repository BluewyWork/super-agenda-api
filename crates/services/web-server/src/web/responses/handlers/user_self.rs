use axum::{extract::State, http::StatusCode};
use lib_database::models::tables::user::UserTable;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::web::{custom::response::ApiResponse, error::Result, utils::token::Claims};

#[derive(Serialize, Deserialize)]
pub struct UserShowPayload {
   username: String,
}

pub async fn show(State(user_table): State<UserTable>, claims: Claims) -> Result<ApiResponse> {
   let user = user_table.find_user_from_object_id(claims.user_id).await?;

   let user_show_payload = UserShowPayload {
      username: user.username,
   };

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: Some(json!({"user": user_show_payload})),
   })
}

pub async fn update() -> Result<ApiResponse> {
   todo!()
}

pub async fn nuke(State(user_table): State<UserTable>, claims: Claims) -> Result<ApiResponse> {
   user_table.delete_user(claims.user_id).await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}
