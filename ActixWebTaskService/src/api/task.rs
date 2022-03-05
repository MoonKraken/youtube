use crate::model::task::Task;
use crate::repository::ddb::DDBRepository;

use actix_web::{
    get, 
    post, 
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    Responder, 
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use serde::{Serialize, Deserialize};
use derive_more::{Display};
//use std::fmt::{Display, Debug};

#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier {
    task_id: String,
}

#[derive(Debug, Display)]
pub enum TaskError {
    TaskNotFound,
    TaskUpdateFailure,
    TaskCreationFailure,
    BadTaskRequest
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::TaskNotFound => StatusCode::NOT_FOUND,
            TaskError::TaskUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            TaskError::TaskCreationFailure => StatusCode::FAILED_DEPENDENCY,
            TaskError::BadTaskRequest => StatusCode::BAD_REQUEST
        }
    }
}

#[get("/task/{task_id}")]
pub async fn get_task(
    ddb_repo: Data<DDBRepository>, 
    task_identifier: Path<TaskIdentifier>
) -> Result<Json<Task>, TaskError> {
    let tsk = ddb_repo.get_task(
        task_identifier.into_inner().task_id
    ).await;

    match tsk {
        Some(tsk) => Ok(Json(tsk)),
        None => Err(TaskError::TaskNotFound)
    }
}

#[post("/task")]
pub async fn submit_task() -> impl Responder {
    HttpResponse::Ok().body("create_task")
}

#[put("/task/{task_id}/start")]
pub async fn start_task() -> impl Responder {
    HttpResponse::Ok().body("")
}

#[put("/task/{task_id}/complete")]
pub async fn complete_task() -> impl Responder {
    HttpResponse::Ok().body("")
}

#[put("/task/{task_id}/pause")]
pub async fn pause_task() -> impl Responder {
    HttpResponse::Ok().body("")
}