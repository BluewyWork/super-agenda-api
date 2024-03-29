use axum::{
   extract::Request,
   http::StatusCode,
   middleware::Next,
   response::{IntoResponse, Response},
};
use serde_json::json;

use crate::{models::api::Answer, utils::jwt::verify_token};

pub async fn guest_middleware(request: Request, next: Next) -> Response {
   let token = match request.headers().get("Authorization") {
      Some(token_wrapped) => match token_wrapped.to_str() {
         Ok(token) => token.to_string(),
         Err(_) => {
            return Answer {
               data: "Invalid Token".into(),
               status: StatusCode::UNAUTHORIZED,
               ok: false,
            }
            .into_response();
         },
      },
      None => {
         return Answer {
            data: "Invalid Token".into(),
            status: StatusCode::UNAUTHORIZED,
            ok: false,
         }
         .into_response();
      },
   };

   let jwt_payload = match verify_token(token) {
      Ok(claims) => claims,
      Err(_) => {
         return Answer {
            data: "Invalid Token".into(),
            status: StatusCode::UNAUTHORIZED,
            ok: false,
         }
         .into_response();
      },
   };

   let mut request = request;
   request.extensions_mut().insert(jwt_payload);

   next.run(request).await
}

pub async fn response_mapper(response: Response) -> Answer {
   let ok = response.status() < StatusCode::from_u16(400).unwrap();

   Answer {
      data: json!({}),
      status: response.status(),
      ok: ok,
   }
}
