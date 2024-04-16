use axum::{extract::State, http::StatusCode, routing::post, Router};
use lib_database::user_table::UserTable;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::web::{
   custom_extractors::Json,
   custom_response::ApiResponse,
   error::{Error, Result},
   password_manager::matches,
   token::create_token,
};

pub fn routes(user_table: UserTable) -> Router {
   Router::new()
      .route("/login", post(api_login_handler))
      .with_state(user_table)
}

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
   username: String,
   password: String,
}

pub async fn api_login_handler(
   State(user_table): State<UserTable>,
   Json(login_payload): Json<LoginPayload>,
) -> Result<ApiResponse> {
   let LoginPayload {
      username,
      password: password_clear,
   } = login_payload;

   let user = user_table.find_user(&username).await?;

   let is_same = matches(password_clear, &user.hashed_password)?;

   if !is_same {
      return Err(Error::PasswordDoesNotMatch);
   }

   let token = create_token(username)?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: Some(json!({"token": token})),
   })
}
