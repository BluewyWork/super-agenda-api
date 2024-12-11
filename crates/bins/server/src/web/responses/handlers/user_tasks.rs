use std::sync::Arc;

use axum::{
   extract::{Path, State},
   http::StatusCode,
};
use database::models::tables::user_data::Task;
use serde_json::json;

use crate::{
   error::AppResult,
   web::{
      custom::{extractors::Json, response::ApiResponse},
      utils::token::Claims,
   },
   AppState,
};

pub async fn create(
   State(app_state): State<Arc<AppState>>,
   claims: Claims,
   Json(task): Json<Task>,
) -> AppResult<ApiResponse> {
   app_state
      .user_data_table
      .create_task(claims.user_id, task)
      .await?;

   Ok(ApiResponse {
      status_code: StatusCode::CREATED,
      message: None,
      data: None,
   })
}

pub async fn show_list(
   State(app_state): State<Arc<AppState>>,
   claims: Claims,
) -> AppResult<ApiResponse> {
   let task_list = app_state
      .user_data_table
      .get_task_list(claims.user_id)
      .await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: Some(json!(task_list)),
   })
}

pub async fn update(
   State(app_state): State<Arc<AppState>>,
   claims: Claims,
   Json(payload): Json<Task>,
) -> AppResult<ApiResponse> {
   app_state
      .user_data_table
      .update_task(claims.user_id, payload)
      .await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}

pub async fn update_list(
   State(app_state): State<Arc<AppState>>,
   claims: Claims,
   Json(payload): Json<Vec<Task>>,
) -> AppResult<ApiResponse> {
   app_state
      .user_data_table
      .update_task_list(claims.user_id, payload)
      .await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}

pub async fn delete(
   State(app_state): State<Arc<AppState>>,
   claims: Claims,
   Path(id): Path<String>,
) -> AppResult<ApiResponse> {
   app_state
      .user_data_table
      .delete_task(claims.user_id, id)
      .await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}
