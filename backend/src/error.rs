use std::error::Error;

use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

pub struct AppError (Box<dyn Error>);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        tracing::error!("{}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { message: self.0.to_string() })).into_response()
    }
}

impl<E: Error + 'static> From<E> for AppError {
    fn from(error: E) -> Self {
        AppError(Box::new(error))
    }
}
