use axum::{routing::post, Router};

use crate::controllers;

pub fn user_routes() -> Router {
   Router::new()
      .route("/auth/login", post(controllers::user::auth::login))
      .route("/auth/register", post(controllers::user::auth::register))
}
