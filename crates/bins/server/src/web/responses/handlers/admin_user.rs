use std::{str::FromStr, sync::Arc};

use axum::{
   extract::{Path, State},
   http::StatusCode,
};
use database::models::tables::{
   user::{User, UserForUpdate},
   user_data::{Membership, TaskStatus, UserDataForUpdate},
};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
   error::{AppError, AppResult},
   web::{
      custom::{extractors::Json, response::ApiResponse},
      utils::password::hash_password,
   },
   AppState,
};

#[derive(Deserialize, Serialize)]
pub struct UserForAdminView {
   pub id: ObjectId,
   pub username: String,
   pub tasks_statistics: TasksStatistics,
   pub membership: Membership,
}

#[derive(Deserialize, Serialize)]
pub struct TasksStatistics {
   pub num_not_started: i64,
   pub num_ongoing: i64,
   pub num_completed: i64,
}

pub async fn show_user_list(State(app_state): State<Arc<AppState>>) -> AppResult<ApiResponse> {
   let user_list = app_state.user_table.find_all_users().await?;

   let mut user_for_admin_view_list: Vec<UserForAdminView> = Vec::new();

   for user in user_list {
      let task_list = app_state.user_data_table.get_task_list(user.id).await?;
      let membership = app_state.user_data_table.get_membership(user.id).await?;

      let tasks_statistics = TasksStatistics {
         num_not_started: task_list
            .iter()
            .filter(|&task| task.status == TaskStatus::NotStarted)
            .count() as i64,
         num_ongoing: task_list
            .iter()
            .filter(|&task| task.status == TaskStatus::Ongoing)
            .count() as i64,
         num_completed: task_list
            .iter()
            .filter(|&task| task.status == TaskStatus::Completed)
            .count() as i64,
      };

      user_for_admin_view_list.push(UserForAdminView {
         id: user.id,
         username: user.username,
         tasks_statistics,
         membership,
      });
   }

   Ok(ApiResponse {
      message: None,
      status_code: StatusCode::OK,
      data: Some(json!(user_for_admin_view_list)),
   })
}

#[derive(Serialize, Deserialize)]
pub struct UserStuffForUpdate {
   pub username: Option<String>,
   pub membership: Option<Membership>,
}

pub async fn update_user(
   State(app_state): State<Arc<AppState>>,
   Path(id): Path<String>,
   Json(user_stuff_for_update): Json<UserStuffForUpdate>,
) -> AppResult<ApiResponse> {
   let id = ObjectId::from_str(&id)?;

   let user_data_for_update = UserDataForUpdate {
      membership: user_stuff_for_update.membership,
      last_modified: None,
   };

   let user_for_update = UserForUpdate {
      username: user_stuff_for_update.username,
   };

   app_state
      .user_table
      .update_user(id, user_for_update)
      .await?;

   app_state
      .user_data_table
      .update_user_data(id, user_data_for_update)
      .await?;

   Ok(ApiResponse {
      message: None,
      status_code: StatusCode::OK,
      data: None,
   })
}

#[derive(Deserialize)]
pub struct UserForCreate {
   #[serde(rename = "_id")]
   pub id: ObjectId,
   pub username: String,
   pub password: String,
}

pub async fn create_user(
   State(app_state): State<Arc<AppState>>,
   Json(user_for_create): Json<UserForCreate>,
) -> AppResult<ApiResponse> {
   if user_for_create.username.len() < 5 {
      return Err(AppError::UsernameTooShort);
   }

   if (app_state
      .user_table
      .find_user_from_username(&user_for_create.username)
      .await)
      .is_ok()
   {
      return Err(AppError::UsernameIsTaken);
   };

   if user_for_create.password.len() < 5 {
      return Err(AppError::PasswordTooShort);
   }

   app_state
      .user_table
      .create_user(User {
         id: user_for_create.id,
         username: user_for_create.username,
         hashed_password: hash_password(&user_for_create.password)?,
      })
      .await?;

   app_state
      .user_data_table
      .initialize_userdata(user_for_create.id)
      .await?;

   Ok(ApiResponse {
      message: None,
      status_code: StatusCode::OK,
      data: None,
   })
}

pub async fn delete_user(
   State(app_state): State<Arc<AppState>>,
   Path(id): Path<String>,
) -> AppResult<ApiResponse> {
   app_state
      .user_table
      .delete_user(ObjectId::from_str(&id)?)
      .await?;

   Ok(ApiResponse {
      message: None,
      status_code: StatusCode::OK,
      data: None,
   })
}
