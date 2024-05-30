use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use serde_json::json;

use crate::{
   web::{custom::response::ApiResponse, error::Result, utils::token::Claims}, AppState,
};

pub async fn show_user_list(
   State(api_state): State<Arc<AppState>>,
   claims: Claims,
) -> Result<ApiResponse> {
   let user_list = api_state.user_table.find_all_users().await?;

   Ok(ApiResponse {
      message: None,
      status_code: StatusCode::OK,
      data: Some(json!(user_list)),
   })
}
