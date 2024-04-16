use axum::{response::{IntoResponse, Response}, Json};
use serde_json::json;

use crate::web::error::Error;

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
