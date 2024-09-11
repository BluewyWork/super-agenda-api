use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use mongodb::bson::oid::ObjectId;
use serde_json::json;

use crate::{web::{custom::{extractors::Json, response::ApiResponse}, error::Result, utils::token::Claims}, AppState};

pub async fn create(
   State(app_state): State<Arc<AppState>>,
   claims: Claims,
   Json(object_id): Json<ObjectId>,
) -> Result<ApiResponse> {
   app_state
      .user_data_table
      .add_deleted_task(object_id, claims.user_id)
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
) -> Result<ApiResponse> {
   let task_list = app_state
      .user_data_table
      .get_deleted_task_list(claims.user_id)
      .await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: Some(json!(task_list)),
   })
}
