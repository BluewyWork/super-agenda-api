pub mod web {
   pub mod custom {
      pub mod extractors;
      pub mod response;
   }
   pub mod responses {
      pub mod handlers {
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
   pub mod error;
   pub mod routes;
}

use axum::{extract::FromRef, middleware::map_response, Router};
use lib_database::models::{
   database::DatabaseManager,
   tables::{user::UserTable, user_data::UserDataTable},
};
use lib_utils::constants::{MONGO_DB, MONGO_URI, SERVER_ADDRESS};
use tokio::net::TcpListener;

use crate::web::{error::Result, responses::middlewares::map_response_from_error, routes};

#[tokio::main]
async fn main() -> Result<()> {
   let database_manager = DatabaseManager::from(&MONGO_URI, &MONGO_DB).await.unwrap();

   let user_table = UserTable::from(database_manager.clone());
   let user_data_table = UserDataTable::from(database_manager);

   let app_state = AppState {
      api_state: ApiState {
         user_table,
         user_data_table,
      },
   };

   let app = Router::new()
      .nest("/api/user", routes::user_routes(app_state))
      .layer(map_response(map_response_from_error));

   let listener = TcpListener::bind(SERVER_ADDRESS.to_string()).await.unwrap();

   println!("Server running on {}", *SERVER_ADDRESS);

   axum::serve(listener, app).await.unwrap();

   Ok(())
}

#[derive(Clone)]
pub struct AppState {
   api_state: ApiState,
}

#[derive(Clone)]
pub struct ApiState {
   user_table: UserTable,
   user_data_table: UserDataTable,
}

impl FromRef<AppState> for ApiState {
   fn from_ref(app_state: &AppState) -> ApiState {
      app_state.api_state.clone()
   }
}
