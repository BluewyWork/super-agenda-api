use axum::{extract::State, http::StatusCode};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
   web::{custom::response::ApiResponse, error::Result, utils::token::Claims},
   ApiState,
};

#[derive(Serialize, Deserialize)]
pub struct UserShowPayload {
   username: String,
}

pub async fn show(State(api_state): State<ApiState>, claims: Claims) -> Result<ApiResponse> {
   let user = api_state
      .user_table
      .find_user_from_object_id(claims.user_id)
      .await?;

   let user_show_payload = UserShowPayload {
      username: user.username,
   };

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: Some(json!(user_show_payload)),
   })
}

pub async fn update() -> Result<ApiResponse> {
   todo!()
}

pub async fn nuke(State(api_state): State<ApiState>, claims: Claims) -> Result<ApiResponse> {
   api_state.user_table.delete_user(claims.user_id).await?;
   api_state.user_data_table.delete(claims.user_id).await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}
