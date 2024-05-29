use axum::extract::State;
use axum::http::StatusCode;
use serde_json::json;

use crate::web::custom::response::ApiResponse;
use crate::web::error::Result;
use crate::web::utils::token::Claims;
use crate::ApiState;

pub async fn show_user_list(State(api_state): State<ApiState>, claims: Claims) -> Result<ApiResponse> {
   let user_list = api_state.user_table.find_all_users().await?;

   Ok(
      ApiResponse{
         message: None,
         status_code: StatusCode::OK,
         data: Some(json!(user_list))
      }
   )
}
