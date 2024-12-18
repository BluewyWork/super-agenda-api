use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use database::models::tables::user_data::Membership;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
   error::AppResult,
   web::{custom::response::ApiResponse, utils::token::Claims},
   AppState,
};

#[derive(Serialize, Deserialize)]
pub struct UserShowPayload {
   username: String,
   membership: Membership,
}

pub async fn show(
   State(app_state): State<Arc<AppState>>,
   claims: Claims,
) -> AppResult<ApiResponse> {
   let user = app_state
      .user_table
      .find_user_from_object_id(claims.user_id)
      .await?;

   let membership = app_state
      .user_data_table
      .get_membership(claims.user_id)
      .await?;

   let user_show_payload = UserShowPayload {
      username: user.username,
      membership,
   };

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: Some(json!(user_show_payload)),
   })
}

pub async fn update() -> AppResult<ApiResponse> {
   todo!()
}

pub async fn nuke(
   State(app_state): State<Arc<AppState>>,
   claims: Claims,
) -> AppResult<ApiResponse> {
   app_state.user_table.delete_user(claims.user_id).await?;
   app_state.user_data_table.delete(claims.user_id).await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}
