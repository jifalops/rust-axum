use aide::operation::OperationIo;
use axum::{extract::FromRequest, response::IntoResponse};
use axum_jsonschema::JsonSchemaRejection;
use serde::Serialize;
use serde_json::json;

use crate::AppError;

#[derive(FromRequest, OperationIo)]
#[from_request(via(axum_jsonschema::Json), rejection(AppError))]
#[aide(
    input_with = "axum_jsonschema::Json<T>",
    output_with = "axum_jsonschema::Json<T>",
    json_schema
)]
pub struct Json<T>(pub T);

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

impl From<JsonSchemaRejection> for AppError {
    fn from(rejection: JsonSchemaRejection) -> Self {
        match rejection {
            JsonSchemaRejection::Json(j) => Self::Validation(j.to_string()),
            JsonSchemaRejection::Serde(_) => Self::Validation("invalid request".to_string()),
            JsonSchemaRejection::Schema(s) => Self::Validation(json!(s).to_string()),
        }
    }
}