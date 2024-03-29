use axum::{extract::Request, http::StatusCode};

use crate::{models::api::Answer, utils::database::mongodb_connection};

pub async fn update_profile(request: Request) -> Answer {
   let mongodb = match mongodb_connection().await {
      Ok(mongodb) => mongodb,
      Err(_) => {
         return Answer {
            data: "...".into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            ok: false,
         }
      },
   };

   Answer {
      data: "placeholder".into(),
      status: StatusCode::OK,
      ok: true,
   }
}
