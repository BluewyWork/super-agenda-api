use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use database::models::tables::user_data::Membership;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
   web::{custom::response::ApiResponse},
   AppState,
 error::AppResult
};

#[derive(Deserialize, Serialize)]
pub struct UserForAdminView {
   pub id: ObjectId,
   pub username: String,
   pub tasks_size: i64,
   pub membership: Membership
}

pub async fn show_user_list(State(app_state): State<Arc<AppState>>) -> AppResult<ApiResponse> {
   let user_list = app_state.user_table.find_all_users().await?;

   let mut user_for_admin_view_list: Vec<UserForAdminView> = Vec::new();

   for user in user_list {
      let task_list = app_state.user_data_table.get_task_list(user.id).await?;
      let membership = app_state.user_data_table.get_membership(user.id).await?;

      user_for_admin_view_list.push(UserForAdminView {
         id: user.id,
         username: user.username,
         tasks_size: task_list.len() as i64,
         membership,
      });
   }

   Ok(ApiResponse {
      message: None,
      status_code: StatusCode::OK,
      data: Some(json!(user_for_admin_view_list)),
   })
}

pub async fn update_user_membership(State(app_state): State<Arc<AppState>>) -> AppResult<ApiResponse> {
   todo!()
}
