use axum::{extract::State, http::StatusCode, Json};
use database::{manager::DatabaseManager, repositories::user::UserRepository};
use serde_json::Value;

use crate::web::{
   error::{Error::Placeholder, Result},
   response::ApiResponse,
};

pub async fn register(
   State(user_repository): State<UserRepository>,
) -> Result<ApiResponse> {
   todo!()
}

pub async fn login() -> Result<ApiResponse> {
   todo!()
}
