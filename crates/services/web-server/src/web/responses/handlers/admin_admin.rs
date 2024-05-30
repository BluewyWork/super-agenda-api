use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use lib_database::models::tables::admin::Admin;
use mongodb::bson::oid::ObjectId;
use serde_json::json;

use crate::{
   web::{
      custom::{extractors::Json, response::ApiResponse},
      error::Result,
   },
   AppState,
};

pub async fn new(
   State(app_state): State<Arc<AppState>>,
   Json(admin): Json<Admin>,
) -> Result<ApiResponse> {
   app_state.admin_table.create_admin(admin).await?;

   Ok(ApiResponse {
      status_code: StatusCode::CREATED,
      message: None,
      data: None,
   })
}

pub async fn show_all(State(app_state): State<Arc<AppState>>) -> Result<ApiResponse> {
   let admin_list = app_state.admin_table.find_all().await?;

   Ok(ApiResponse {
      status_code: StatusCode::CREATED,
      message: None,
      data: Some(json!(admin_list)),
   })
}

pub async fn update(
   State(app_state): State<Arc<AppState>>,
   Json(admin): Json<Admin>,
) -> Result<ApiResponse> {
   app_state.admin_table.update_admin(admin).await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}

pub async fn delete(
   State(app_state): State<Arc<AppState>>,
   Json(admin_id): Json<ObjectId>,
) -> Result<ApiResponse> {
   app_state.admin_table.delete_admin(admin_id).await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}
