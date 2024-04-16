use auth::decode_token;
use axum::{
   extract::Request,
   http::StatusCode,
   middleware::Next,
   response::{IntoResponse, Response},
   Json,
};
use serde_json::json;
use utils::constants::JWT_SECRET;

use super::error::Error;

pub async fn authenticate_guest(mut request: Request, next: Next) -> Response {
   let mut placeholder = StatusCode::FORBIDDEN.into_response();

   let wrapped_token = request
      .headers()
      .get("Authorization")
      .map(|header| header.to_str());

   match wrapped_token {
      Some(Ok(token)) => match decode_token(JWT_SECRET.to_string(), token.to_string()) {
         Ok(claims) => {
            request.extensions_mut().insert(claims);
            return next.run(request).await;
         },
         Err(_err) => {
            placeholder.extensions_mut().insert(Error::Placeholder);
         },
      },
      Some(Err(_)) => {
         placeholder.extensions_mut().insert(Error::Placeholder);
      },
      None => {
         placeholder.extensions_mut().insert(Error::Placeholder);
      },
   }

   placeholder
}

pub async fn map_response_from_error(response: Response) -> Response {
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
