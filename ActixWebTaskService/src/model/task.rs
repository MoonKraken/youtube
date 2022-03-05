use serde::Serialize;
use uuid::Uuid;
use strum_macros::{EnumString, Display};

#[derive(Serialize, EnumString, Display)]
pub enum TaskState {
    NotStarted,
    InProgress,
    Completed,
    Paused,
    Failed
}

#[derive(Serialize)]
pub struct Task {
    pub user_uuid: String,
    pub task_uuid: String,
    pub task_type: String,
    pub state: TaskState,
    pub source_file: String,
    pub result_file: Option<String>
}

impl Task {
    pub fn new(user_uuid: String, task_type: String, source_file: String) -> Task {
        Task {
            user_uuid,
            task_uuid: Uuid::new_v4().to_string(),
            task_type,
            state: TaskState::NotStarted,
            source_file,
            result_file: None
        }
    }
}