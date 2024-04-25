use axum::{
   middleware::from_fn,
   routing::{delete, get, patch, post},
   Router,
};
use lib_database::models::tables::{user::UserTable, user_data::UserDataTable};

use super::responses::{
   handlers::{
      user_auth, user_self,
      user_tasks::{self},
   },
   middlewares::authenticate_guest,
};

pub fn user_routes(user_table: UserTable, task_groups_table: UserDataTable) -> Router {
   let user_auth_routes = Router::new()
      .route("/register", post(user_auth::register))
      .route("/login", post(user_auth::login))
      .with_state(user_table.clone());

   let user_self_routes = Router::new()
      .route("/show", get(user_self::show))
      .route("/delete", delete(user_self::nuke))
      .layer(from_fn(authenticate_guest))
      .with_state(user_table);

   let user_task_routes = Router::new()
      .route("/create", post(user_tasks::create))
      .route("/show", get(user_tasks::show_list))
      .route("/update", post(user_tasks::update))
      .route("/update/list", post(user_tasks::update_list))
      .route("/delete", patch(user_tasks::delete))
      .layer(from_fn(authenticate_guest))
      .with_state(task_groups_table);

   Router::new()
      .nest("/auth", user_auth_routes)
      .nest("/self", user_self_routes)
      .nest("/task", user_task_routes)
}
