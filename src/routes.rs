use axum::{routing::post, Router};

use crate::controllers;

pub fn auth_user() -> Router {
   Router::new()
      .route("/auth/user/login", post(controllers::user::auth::login))
      .route(
         "/auth/user/register",
         post(controllers::user::auth::register),
      )
}
