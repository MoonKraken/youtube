use crate::model::task::{Task, TaskState};
use log::error;
use std::str::FromStr;
use std::collections::HashMap;

pub struct DB {}

pub struct DBError;

impl DB {
    pub fn init() -> DB {
        todo!()
    }

    pub async fn put_task(&self, task: Task) -> Result<(), DBError> {
        todo!()
    }
}