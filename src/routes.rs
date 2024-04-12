use axum::{
   middleware::from_fn,
   routing::{get, post},
   Router,
};

use crate::{
   controllers,
   middleware::authenticate_guest,
};

pub fn user_routes() -> Router {
   let user_auth_route = Router::new()
      .route("/login", post(controllers::user::auth::login))
      .route("/register", post(controllers::user::auth::register));

   let user_profile_route = Router::new()
      .route("/me", get(controllers::user::profile::show))
      .route("/update", post(controllers::user::profile::update))
      .layer(from_fn(authenticate_guest));

   Router::new()
      .nest("/auth", user_auth_route)
      .nest("/profile", user_profile_route)
}
