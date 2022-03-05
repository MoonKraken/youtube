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

struct DDBError;

fn item_value(key: &str, item: &HashMap<String, AttributeValue>) -> Result<String, DDBError> {
    match item.get(key) {
        Some(value) => match value.as_s() {
            Ok(val) => Ok(val.clone()),
            Err(_) => Err(DDBError)
        },
        None => Err(DDBError)
    }
}

fn item_to_task(item: &HashMap<String, AttributeValue>) -> Result<Task, DDBError> {
    let state = match TaskState::from_str(item_value("state", item)?.as_str()) {
        Ok(state) => state,
        Err(_) => return Err(DDBError)
    };

    Ok(Task {
        user_uuid: item_value("pK", item)?,
        task_uuid: item_value("sK", item)?,
        task_type: item_value("task_type", item)?,
        state,
        source_file: item_value("source_file", item)?,
        result_file: Some(item_value("result_file", item)?)
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
        
        let request = self.client.put_item()
            .table_name(&self.table_name)
            .item("pK", AttributeValue::S(String::from(task.user_uuid)))
            .item("sK", AttributeValue::S(String::from(task.task_uuid)))
            .item("task_type", AttributeValue::S(String::from(task.task_type)))
            .item("state", AttributeValue::S(task.state.to_string()))
            .item("source_file", AttributeValue::S(String::from(task.source_file)));
        
        if let Some(result_file) = task.result_file {
            request.item("result_file", AttributeValue::S(String::from(result_file)));
        }

        match request.send().await {
            Ok(res) => Ok(()),
            Err(err) => Err(DDBError)
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

        match res {
            Ok(output) => {
                match output.items {
                    Some(items) => {
                        let item = &items.first()?;
                        error!("{:?}", &item);
                        return match item_to_task(item) {
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