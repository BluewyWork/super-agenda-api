use axum::{
   routing::{delete, get, post},
   Router,
};
use database::{manager::DatabaseManager, repositories::user::UserRepository};
use utils::constants::{MONGO_DB, MONGO_URL};

use crate::web::controllers::{admin, user};

pub async fn user_routes() -> Router {
   let database_manager = match DatabaseManager::from(&MONGO_URL, &MONGO_DB).await {
      Ok(database_manager) => database_manager,
      Err(_) => panic!(),
   };

   let user_repository =  UserRepository::from(database_manager.users_collection());

   let user_auth_routes = Router::new()
      .route("/register", post(user::auth::register))
      .route("/login", post(user::auth::login))
      .with_state(user_repository);

   let user_profile_routes = Router::new()
      .route("/show", get(user::profile::show))
      .route("/update", post(user::profile::update))
      .route("/delete", delete(user::profile::delete));

   Router::new()
      .nest("/auth", user_auth_routes)
      .nest("/profile", user_profile_routes)
}

pub fn admin_routes() -> Router {
   let admin_auth_routes = Router::new()
      .route("/register", post(admin::auth::register))
      .route("/login", post(admin::auth::login));

   Router::new().nest("/auth", admin_auth_routes)
}
