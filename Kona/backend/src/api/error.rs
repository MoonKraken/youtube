use actix_web::{
    error::ResponseError,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use serde::{Serialize};
use strum::Display;

#[derive(Debug, Display, Serialize)]
pub enum BlogError {
    BlogNotFound,
    PostCreationFailed
}

impl ResponseError for BlogError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            BlogError::BlogNotFound => StatusCode::NOT_FOUND,
            BlogError::PostCreationFailed => StatusCode::FAILED_DEPENDENCY,
        }
    }
}