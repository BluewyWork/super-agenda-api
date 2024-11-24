pub mod web {
   pub mod custom {
      pub mod extractors;
      pub mod response;
   }
   pub mod responses {
      pub mod handlers {
         pub mod admin_admin;
         pub mod admin_auth;
         pub mod admin_user;
         pub mod user_auth;
         pub mod user_self;
         pub mod user_tasks;
      }
      pub mod middlewares;
   }
   pub mod utils {
      pub mod password;
      pub mod token;
   }
}
pub mod error;

use axum::{
   middleware::{from_fn, map_response},
   routing::{delete, get, patch, post},
   Router,
};
use database::models::{
   database::DatabaseManager,
   tables::{admin::AdminTable, user::UserTable, user_data::UserDataTable},
};
use std::sync::Arc;
use tokio::net::TcpListener;
use utils::constants::{MONGO_DB, MONGO_URI, SERVER_ADDRESS};
use web::responses::{
   handlers::{
      admin_admin::{self, new},
      admin_auth, admin_user, user_auth, user_self, user_tasks,
   },
   middlewares::authenticate_user_or_admin,
};

use crate::error::AppResult;
use crate::web::responses::middlewares::map_response_from_error;

#[tokio::main]
async fn main() -> AppResult<()> {
   let database_manager = DatabaseManager::from(&MONGO_URI, &MONGO_DB).await.unwrap();
   let admin_table = AdminTable::from(database_manager.clone());
   let user_table = UserTable::from(database_manager.clone());
   let user_data_table = UserDataTable::from(database_manager);

   let app_state = Arc::new(AppState {
      admin_table,
      user_table,
      user_data_table,
   });

   let app = Router::new()
      .nest(
         "/api",
         Router::new()
            .route("/auth/login/admin", post(admin_auth::login))
            .route("/auth/login/user", post(user_auth::login))
            .route("/auth/register/user", post(user_auth::register))
            .with_state(Arc::clone(&app_state)),
      )
      .nest(
         "/api",
         Router::new()
            .route("/admins", post(admin_admin::new))
            .route("/admins", get(admin_admin::show_all))
            .route("/admins/:id", patch(admin_admin::update))
            .route("/admins/:id", delete(admin_admin::delete))
            .route("/users", get(admin_user::show_user_list))
            .route("/users/:id", patch(admin_user::update_user))
            .route("/claims/user", get(user_self::show))
            .route("/claims/user", patch(user_self::update))
            .route("/claims/user", delete(user_self::nuke))
            .route("/claims/tasks", post(user_tasks::create))
            .route("/claims/tasks", get(user_tasks::show_list))
            .route("/claims/tasks", patch(user_tasks::update))
            .route("/claims/tasks", delete(user_tasks::delete))
            .layer(from_fn(authenticate_user_or_admin))
            .with_state(Arc::clone(&app_state)),
      )
      .layer(map_response(map_response_from_error));

   let listener = TcpListener::bind(SERVER_ADDRESS.to_string()).await.unwrap();
   println!("Server running on {}", *SERVER_ADDRESS);
   axum::serve(listener, app).await.unwrap();

   Ok(())
}

pub struct AppState {
   admin_table: AdminTable,
   user_table: UserTable,
   user_data_table: UserDataTable,
}
