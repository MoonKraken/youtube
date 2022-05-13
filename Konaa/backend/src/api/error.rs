use std::fmt::{Display, Formatter};
use actix_web::{
    error::ResponseError,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use serde::{Serialize};
use strum::Display;
//use derive_more::Display;

#[derive(Debug, Display, Serialize)]
pub enum BlogError {
    BlogNotFound,
    PostCreationFailed,
    DateTimeParseError
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
            BlogError::DateTimeParseError => StatusCode::BAD_REQUEST
        }
    }
}