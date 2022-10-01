use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_config::Config;
use crate::model::task::{Task, TaskState};
use log::error;
use std::str::FromStr;
use std::collections::HashMap;

pub struct DDBRepository {
    client: Client,
    table_name: String
}

pub struct DDBError;

fn required_item_value(key: &str, item: &HashMap<String, AttributeValue>) -> Result<String, DDBError> {
    match item_value(key, item) {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Err(DDBError),
        Err(DDBError) => Err(DDBError)
    }
}

fn item_value(key: &str, item: &HashMap<String, AttributeValue>) -> Result<Option<String>, DDBError> {
    match item.get(key) {
        Some(value) => match value.as_s() {
            Ok(val) => Ok(Some(val.clone())),
            Err(_) => Err(DDBError)
        },
        None => Ok(None)
    }
}

fn item_to_task(item: &HashMap<String, AttributeValue>) -> Result<Task, DDBError> {
    let state: TaskState = match TaskState::from_str(required_item_value("state", item)?.as_str()) {
        Ok(value) => value,
        Err(_) => return Err(DDBError)
    };

    let result_file = item_value("result_file", item)?;

    Ok(Task {
        user_uuid: required_item_value("pK", item)?,
        task_uuid: required_item_value("sK", item)?,
        task_type: required_item_value("task_type", item)?,
        state,
        source_file: required_item_value("source_file", item)?,
        result_file
    })
}

impl DDBRepository {
    pub fn init(table_name: String, config: Config) -> DDBRepository {
        let client = Client::new(&config);
        DDBRepository {
            table_name,
            client
        }
    }

    pub async fn put_task(&self, task: Task) -> Result<(), DDBError> {
        let mut request = self.client.put_item()
            .table_name(&self.table_name)
            .item("pK", AttributeValue::S(String::from(task.user_uuid)))
            .item("sK", AttributeValue::S(String::from(task.task_uuid)))
            .item("task_type", AttributeValue::S(String::from(task.task_type)))
            .item("state", AttributeValue::S(task.state.to_string()))
            .item("source_file", AttributeValue::S(String::from(task.source_file)));
        
        if let Some(result_file) = task.result_file {
            request = request.item("result_file", AttributeValue::S(String::from(result_file)));
        }

        match request.send().await {
            Ok(_) => Ok(()),
            Err(_) => Err(DDBError)
        }
    }

    pub async fn get_task(&self, task_id: String) -> Option<Task> {
        let tokens:Vec<String> = task_id
            .split("_")
            .map(|x| String::from(x))
            .collect();
        let user_uuid = AttributeValue::S(tokens[0].clone());
        let task_uuid = AttributeValue::S(tokens[1].clone());
        
        let res = self.client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("#pK = :user_id and #sK = :task_uuid")
            .expression_attribute_names("#pK", "pK")
            .expression_attribute_names("#sK", "sK")
            .expression_attribute_values(":user_id", user_uuid)
            .expression_attribute_values(":task_uuid", task_uuid)
            .send()
            .await;

        return match res {
            Ok(output) => {
                match output.items {
                    Some(items) => {
                        let item = &items.first()?;
                        error!("{:?}", &item);
                        match item_to_task(item) {
                            Ok(task) => Some(task),
                            Err(_) => None
                        }
                    },
                    None => {
                        None
                    }
                }
            },
            Err(error) => {
                error!("{:?}", error);
                None
            }
        }
    }
}