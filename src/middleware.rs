use axum::{
   extract::Request,
   http::StatusCode,
   middleware::Next,
   response::{IntoResponse, Response},
};

use crate::{models::api::Answer, utils::jwt::verify_token};

pub async fn guest_middleware(request: Request, next: Next) -> Response {
   let token = match request.headers().get("Authorization") {
      Some(token_wrapped) => match token_wrapped.to_str() {
         Ok(token) => token.to_string(),
         Err(_) => {
            let answer = Answer {
               json: "Invalid Token".into(),
               status: StatusCode::UNAUTHORIZED,
               ok: false,
            };

            return answer.into_response();
         },
      },
      None => {
         let answer = Answer {
            json: "Invalid Token".into(),
            status: StatusCode::UNAUTHORIZED,
            ok: false,
         };

         return answer.into_response();
      },
   };

   if let Err(_) = verify_token(token) {
      let answer = Answer {
         json: "Invalid Token".into(),
         status: StatusCode::UNAUTHORIZED,
         ok: false,
      };

      return answer.into_response();
   }

   let response = next.run(request).await;

   response
}
