use axum::{
   extract::Request,
   middleware::Next,
   response::{IntoResponse, Response},
};
use serde_json::{json, to_value};

use crate::response::{error::Error, success::Success};
use crate::utils::{extractor::Json, jwt::verify_token};

#[allow(dead_code)]
pub async fn middleware_guest_authentication(request: Request, next: Next) -> Response {
   let token = match request.headers().get("Authorization") {
      Some(token_wrapped) => match token_wrapped.to_str() {
         Ok(token) => token.to_string(),
         Err(_) => return Error::TokenStuff.into_response(),
      },
      None => return Error::TokenStuff.into_response(),
   };

   let jwt_payload = match verify_token(token) {
      Ok(claims) => claims,
      Err(_) => return Error::TokenStuff.into_response(),
   };

   let mut request = request;
   request.extensions_mut().insert(jwt_payload);

   next.run(request).await
}

pub async fn middleware_error_response_mapper(response: Response) -> Response {
   let service_error = response.extensions().get::<Error>();
   let client_status_error = service_error.map(|se| se.client_status_and_error());

   let error_reponse = client_status_error
      .as_ref()
      .map(|(status_code, client_error)| {
         let client_error_body = json!({
            "ok": false,
            "message": client_error.as_ref(),
            "data": {}
         });

         (*status_code, Json(client_error_body)).into_response()
      });

   error_reponse.unwrap_or(response)
}

pub async fn middleware_success_response_mapper(response: Response) -> Response {
   let service_success = response.extensions().get::<Success>();
   let client_status_success = service_success.map(|ss| ss.client_status_and_success());

   let success_response = client_status_success
      .as_ref()
      .map(|(status_code, client_success)| {
         let client_success = to_value(client_success).ok();
         let message = client_success.as_ref().and_then(|v| v.get("message"));
         let data = client_success.as_ref().and_then(|v| v.get("data"));

         let client_success_body = json!({
            "ok": true,
            "message": message,
            "data": data.unwrap_or(&json!({}))
         });

         (*status_code, Json(client_success_body)).into_response()
      });

   success_response.unwrap_or(response)
}
