use axum::{
   extract::{Path, State},
   http::StatusCode,
};
use lib_database::models::tables::user_data::{Task, UserDataTable};
use mongodb::bson::oid::ObjectId;
use serde_json::json;

use crate::web::{
   custom::{extractors::Json, response::ApiResponse},
   error::Result,
   utils::token::Claims,
};

pub async fn create(
   State(user_data_table): State<UserDataTable>,
   claims: Claims,
   Json(task): Json<Task>,
) -> Result<ApiResponse> {
   user_data_table.create_task(claims.user_id, task).await?;

   Ok(ApiResponse {
      status_code: StatusCode::CREATED,
      message: None,
      data: None,
   })
}

pub async fn show(
   State(user_data_table): State<UserDataTable>,
   claims: Claims,
) -> Result<ApiResponse> {
   let task_list = user_data_table.get_task_list(claims.user_id).await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: Some(json!(task_list)),
   })
}

pub async fn update(
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

pub async fn update_list(
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

pub async fn delete(
   State(user_data_table): State<UserDataTable>,
   claims: Claims,
   Path(task_id): Path<ObjectId>,
) -> Result<ApiResponse> {
   user_data_table.delete_task(claims.user_id, task_id).await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}
