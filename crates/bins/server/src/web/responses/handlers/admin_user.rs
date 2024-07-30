use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use serde_json::json;

use crate::{
   web::{custom::response::ApiResponse, error::Result},
   AppState,
};

pub async fn show_user_list(State(app_state): State<Arc<AppState>>) -> Result<ApiResponse> {
   let user_list = app_state.user_table.find_all_users().await?;

   Ok(ApiResponse {
      message: None,
      status_code: StatusCode::OK,
      data: Some(json!(user_list)),
   })
}
