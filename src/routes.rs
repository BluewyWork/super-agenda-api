use axum::{
   routing::{get, post},
   Router,
};

use crate::{controllers, middleware::guest_middleware};

pub fn user_routes() -> Router {
   let user_auth_route = Router::new()
      .route("/login", post(controllers::user::auth::login))
      .route("/register", post(controllers::user::auth::register));

   let user_task_route = Router::new()
      .route("/new", post(controllers::user::task::create_task))
      .layer(axum::middleware::from_fn(guest_middleware));

   let user_test_route = Router::new()
      .route("/hi", get(|| async { println!("hi") }))
      .layer(axum::middleware::from_fn(guest_middleware));

   Router::new()
      .nest("/auth", user_auth_route)
      .nest("/task", user_task_route)
      .nest("/test", user_test_route)
}
