use axum::{
   middleware::from_fn,
   routing::{delete, get, patch, post},
   Router,
};

use super::responses::{
   handlers::{admin_auth, admin_stuff, user_auth, user_self, user_tasks},
   middlewares::authenticate_user_or_admin,
};
use crate::AppState;

pub fn user_routes(app_state: &AppState) -> Router {
   let user_auth_routes = Router::new()
      .route("/register", post(user_auth::register))
      .route("/login", post(user_auth::login))
      .with_state(app_state.clone());

   let user_self_routes = Router::new()
      .route("/show", get(user_self::show))
      .route("/delete", delete(user_self::nuke))
      .layer(from_fn(authenticate_user_or_admin))
      .with_state(app_state.clone());

   let user_task_routes = Router::new()
      .route("/create", post(user_tasks::create))
      .route("/show", get(user_tasks::show_list))
      .route("/update", post(user_tasks::update))
      .route("/update/list", post(user_tasks::update_list))
      .route("/delete", patch(user_tasks::delete))
      .layer(from_fn(authenticate_user_or_admin))
      .with_state(app_state.clone());

   Router::new()
      .nest("/auth", user_auth_routes)
      .nest("/self", user_self_routes)
      .nest("/task", user_task_routes)
}

pub fn admin_routes(app_state: &AppState) -> Router {
   let admin_auth_routes = Router::new()
      .route("/login", post(admin_auth::login))
      .with_state(app_state.clone());

   let admin_stuff_routes = Router::new()
      .route("/show/all", get(admin_stuff::show_user_list))
      .layer(from_fn(authenticate_user_or_admin))
      .with_state(app_state.clone());

   Router::new()
      .nest("/auth", admin_auth_routes)
      .nest("/user", admin_stuff_routes)
}
