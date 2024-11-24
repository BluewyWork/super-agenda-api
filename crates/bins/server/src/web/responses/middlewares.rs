use axum::{
   extract::Request,
   http::StatusCode,
   middleware::Next,
   response::{IntoResponse, Response},
   Json,
};
use serde_json::json;

use crate::{error::AppError, web::utils::token::decrypt_token};

pub async fn map_response_from_error(response: Response) -> Response {
   let service_error = response.extensions().get::<AppError>();
   let client_status_error = service_error.map(|se| se.client_status_and_error());

   let error_reponse = client_status_error
      .as_ref()
      .map(|(status_code, client_error)| {
         let client_error_body = json!({
            "ok": false,
            "error": client_error.as_ref(),
         });

         (*status_code, Json(client_error_body)).into_response()
      });

   let client_error = client_status_error.unzip().1;
   println! {"ERROR => SERVICE: {:?} |---| CLIENT: {:?}", service_error, client_error}

   error_reponse.unwrap_or(response)
}

pub async fn authenticate_user_or_admin(mut request: Request, next: Next) -> Response {
   let mut placeholder = StatusCode::FORBIDDEN.into_response();

   let wrapped_token = request
      .headers()
      .get("Authorization")
      .map(|header| header.to_str());

   match wrapped_token {
      Some(Ok(token)) => match decrypt_token(token.to_string()) {
         Ok(claims) => {
            request.extensions_mut().insert(claims);
            return next.run(request).await;
         },
         Err(err) => {
            placeholder.extensions_mut().insert(err);
         },
      },
      Some(Err(_)) => {
         placeholder.extensions_mut().insert(AppError::TokenNotFound);
      },
      None => {
         placeholder.extensions_mut().insert(AppError::TokenNotFound);
      },
   }

   placeholder
}
