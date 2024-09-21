use axum::{routing::post, Router};
use lib_database::models::{
   database::DatabaseManager,
   tables::{admin::AdminTable, user::UserTable, user_data::UserDataTable},
};

pub fn routes() -> Router {
   Router::new().route("/rpc", post(|| async {}));

   todo!()
}

struct AppState {
   admin_table: AdminTable,
   user_table: UserTable,
   userdata_table: UserDataTable,
}
