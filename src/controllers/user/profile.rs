use axum::{extract::Request, http::StatusCode};

use crate::models::api::Answer;

pub async fn update_profile(request: Request) -> Answer {
   Answer {
      json: "placeholder".into(),
      status: StatusCode::OK,
      ok: true,
   }
}
