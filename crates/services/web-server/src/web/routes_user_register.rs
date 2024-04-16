use axum::{extract::State, http::StatusCode, routing::post, Router};
use lib_database::user_table::UserTable;
use serde::{Deserialize, Serialize};

use crate::web::{
   custom_extractors::Json,
   custom_response::ApiResponse,
   error::{Error, Result},
   password_manager::hash_password,
};

pub fn routes(user_table: UserTable) -> Router {
   Router::new()
      .route("/register", post(api_register_handler))
      .with_state(user_table)
}

#[derive(Serialize, Deserialize)]
pub struct RegisterPayload {
   username: String,
   password: String,
}

pub async fn api_register_handler(
   State(user_table): State<UserTable>,
   Json(register_payload): Json<RegisterPayload>,
) -> Result<ApiResponse> {
   let RegisterPayload {
      username,
      password: password_clear,
   } = register_payload;

   if username.len() < 5 {
      return Err(Error::UsernameTooShort);
   }

   if password_clear.len() < 5 {
      return Err(Error::PasswordTooShort);
   }

   let hashed_password = hash_password(&password_clear)?;
   user_table.create_user(&username, &hashed_password).await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}
