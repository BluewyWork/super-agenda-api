use std::sync::Arc;

use axum::{
   middleware::from_fn,
   routing::{delete, get, patch, post},
   Router,
};

use super::responses::{
   handlers::{admin_admin, admin_auth, admin_user, user_auth, user_self, user_tasks},
   middlewares::authenticate_user_or_admin,
};
use crate::AppState;

pub fn user_routes(app_state: Arc<AppState>) -> Router {
   let user_auth_routes = Router::new()
      .route("/register", post(user_auth::register))
      .route("/login", post(user_auth::login))
      .with_state(Arc::clone(&app_state));

   let user_self_routes = Router::new()
      .route("/show", get(user_self::show))
      .route("/delete", delete(user_self::nuke))
      .layer(from_fn(authenticate_user_or_admin))
      .with_state(Arc::clone(&app_state));

   let user_task_routes = Router::new()
      .route("/create", post(user_tasks::create))
      .route("/show", get(user_tasks::show_list))
      .route("/update", post(user_tasks::update))
      .route("/update/list", post(user_tasks::update_list))
      .route("/delete", patch(user_tasks::delete))
      .layer(from_fn(authenticate_user_or_admin))
      .with_state(Arc::clone(&app_state));

   Router::new()
      .nest("/auth", user_auth_routes)
      .nest("/self", user_self_routes)
      .nest("/task", user_task_routes)
}

pub fn admin_routes(app_state: Arc<AppState>) -> Router {
   let admin_auth_routes = Router::new()
      .route("/login", post(admin_auth::login))
      .with_state(Arc::clone(&app_state));

   let admin_admin_routes = Router::new()
      .route("/new", post(admin_admin::new))
      .route("/show/all", get(admin_admin::show_all))
      .route("/update", post(admin_admin::update))
      .route("/nuke", delete(admin_admin::delete))
      .layer(from_fn(authenticate_user_or_admin))
      .with_state(Arc::clone(&app_state));

   let admin_user_routes = Router::new()
      .route("/show/all", get(admin_user::show_user_list))
      .layer(from_fn(authenticate_user_or_admin))
      .with_state(Arc::clone(&app_state));

   Router::new()
      .nest("/auth", admin_auth_routes)
      .nest("/admin", admin_admin_routes)
      .nest("/user", admin_user_routes)
}
