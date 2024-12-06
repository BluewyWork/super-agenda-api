use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use axum_extra::extract::OptionalQuery;
use database::models::tables::user_data::UserDataForUpdate;
use serde::Deserialize;
use serde_json::{json, Map};

use crate::{
   error::{AppError, AppResult},
   web::{
      custom::{extractors::Json, response::ApiResponse},
      utils::token::Claims,
   },
   AppState,
};

pub async fn update(
   State(app_state): State<Arc<AppState>>,
   claims: Claims,
   Json(payload): Json<UserDataForUpdate>,
) -> AppResult<ApiResponse> {
   app_state
      .user_data_table
      .update_user_data(claims.user_id.to_string(), payload)
      .await?;

   Ok(ApiResponse {
      status_code: StatusCode::OK,
      message: None,
      data: None,
   })
}

#[derive(Deserialize)]
pub struct QueryParams {
   pub fields: Vec<String>,
}

pub async fn get(
   State(app_state): State<Arc<AppState>>,
   claims: Claims,
   OptionalQuery(maybe_queries): OptionalQuery<QueryParams>,
) -> AppResult<ApiResponse> {
   match maybe_queries {
      Some(query) => {
         let mut partial = Map::new();

         for field in &query.fields {
            match field.as_str() {
               "last_modified" => {
                  let result = app_state
                     .user_data_table
                     .get_last_modified(claims.user_id)
                     .await?;

                  partial.insert("last_modified".to_string(), json!(result));
               },

               _ => return Err(AppError::QueryNotValid),
            }
         }

         Ok(ApiResponse {
            message: None,
            status_code: StatusCode::OK,
            data: Some(json!(partial)),
         })
      },

      None => {
         todo!()
      },
   }
}
