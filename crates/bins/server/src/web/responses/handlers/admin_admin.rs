use std::sync::Arc;

use axum::{
   extract::{Path, State},
   http::StatusCode,
};
use database::models::tables::admin::{Admin, AdminForUpdate};
use serde_json::json;

use crate::{
   error::{AppError, AppResult},
   web::{
      custom::{extractors::Json, response::ApiResponse},
      utils::password::hash_password,
   },
   AppState,
};

pub async fn new(
   State(app_state): State<Arc<AppState>>,
   Json(admin_for_create): Json<Admin>,
) -> AppResult<ApiResponse> {
   if admin_for_create.username.len() < 5 {
      return Err(AppError::UsernameTooShort);
   }

   if app_state
      .admin_table
      .find_admin_from_username(&admin_for_create.username)
      .await
      .is_ok()
   {
      return Err(AppError::UsernameIsTaken);
   }

   if admin_for_create.hashed_password.len() < 5 {
      return Err(AppError::PasswordTooShort);
   }

   app_state
      .admin_table
      .create_admin(Admin {
         id: admin_for_create.id,
         username: admin_for_create.username,
         hashed_password: hash_password(&admin_for_create.hashed_password)?,
      })
      .await?;

   Ok(ApiResponse {
      status_code: StatusCode::CREATED,
      message: None,
      data: None,
   })
}

pub async fn show_all(State(app_state): State<Arc<AppState>>) -> AppResult<ApiResponse> {
   let admin_list = app_state.admin_table.find_all().await?;

   Ok(ApiResponse {
      status_code: StatusCode::CREATED,
      message: None,
      data: Some(json!(admin_list)),
   })
}

pub async fn update(
   State(app_state): State<Arc<AppState>>,
   Path(id): Path<String>,
   Json(admin_for_update): Json<AdminForUpdate>,
) -> AppResult<ApiResponse> {
   app_state
      .admin_table
      .update_admin(&id, admin_for_update)
      .await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}

pub async fn delete(
   State(app_state): State<Arc<AppState>>,
   Path(admin_id): Path<String>,
) -> AppResult<ApiResponse> {
   app_state.admin_table.delete_admin(admin_id).await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}
