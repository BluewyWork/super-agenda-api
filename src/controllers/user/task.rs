use axum::{extract::Request, http::StatusCode};

use crate::{models::api::Answer, utils::jwt::Claims};

pub async fn create_task(request: Request) -> Answer {
   let claims = request.extensions().get::<Claims>();

   println!("{:?}", claims);

   return Answer {
      json: "hahaha".into(),
      status: StatusCode::OK,
      ok: true,
   };
}
