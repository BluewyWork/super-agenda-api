use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
   web::{custom::response::ApiResponse, error::Result, utils::token::Claims},
   AppState,
};

#[derive(Serialize, Deserialize)]
pub struct UserShowPayload {
   username: String,
}

pub async fn show(State(app_state): State<Arc<AppState>>, claims: Claims) -> Result<ApiResponse> {
   let user = app_state
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

pub async fn nuke(State(app_state): State<Arc<AppState>>, claims: Claims) -> Result<ApiResponse> {
   app_state.user_table.delete_user(claims.user_id).await?;
   app_state.user_data_table.delete(claims.user_id).await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}
