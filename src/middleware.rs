use axum::{
   extract::Request,
   middleware::Next,
   response::{IntoResponse, Response},
};

use crate::{response, utils::jwt::verify_token};

#[allow(dead_code)]
pub async fn guest_middleware(request: Request, next: Next) -> Response {
   let token = match request.headers().get("Authorization") {
      Some(token_wrapped) => match token_wrapped.to_str() {
         Ok(token) => token.to_string(),
         Err(_) => return response::Error::TokenStuff.into_response(),
      },
      None => return response::Error::TokenStuff.into_response(),
   };

   let jwt_payload = match verify_token(token) {
      Ok(claims) => claims,
      Err(_) => return response::Error::TokenStuff.into_response(),
   };

   let mut request = request;
   request.extensions_mut().insert(jwt_payload);

   next.run(request).await
}
