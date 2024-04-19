use axum::{extract::State, http::StatusCode};
use lib_database::models::tables::user_data::{Task, UserDataTable};
use serde_json::json;

use crate::web::{
   custom::{extractors::Json, response::ApiResponse},
   error::Result,
   utils::token::Claims,
};

pub async fn create() -> Result<ApiResponse> {
   todo!()
}

pub async fn show(
   State(users_task_list_table): State<UserDataTable>,
   claims: Claims,
) -> Result<ApiResponse> {
   let tasks = users_task_list_table.get_task_list(claims.user_id).await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: Some(json!(tasks)),
   })
}

pub async fn update_task(
   State(user_data_table): State<UserDataTable>,
   claims: Claims,
   Json(payload): Json<Task>,
) -> Result<ApiResponse> {
   user_data_table.update_task(claims.user_id, payload).await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}

pub async fn update_task_list(
   State(user_data_table): State<UserDataTable>,
   claims: Claims,
   Json(payload): Json<Vec<Task>>,
) -> Result<ApiResponse> {
   user_data_table
      .update_task_list(claims.user_id, payload)
      .await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}

pub async fn delete_task() -> Result<ApiResponse> {
   todo!()
}
