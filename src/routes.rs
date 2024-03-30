use axum::{routing::post, Router};

use crate::controllers;

pub fn user_routes() -> Router {
   let user_auth_route = Router::new().route("/login", post(controllers::user::auth::login));

   Router::new().nest("/auth", user_auth_route)
}
