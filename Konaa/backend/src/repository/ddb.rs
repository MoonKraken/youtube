use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_config::Config;
use common::model::post::Post;
use common::model::blog::NewBlog;
use log::error;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

pub struct DDBRepository {
    client: Client,
    table_name: String
}

#[derive(Debug)]
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

fn item_to_post(item: &HashMap<String, AttributeValue>) -> Result<Post, DDBError> {
    let title = item_value("title", item)?;

    Ok(Post {
        blog_id: required_item_value("pK", item)?,
        post_id: required_item_value("sK", item)?,
        author_name: required_item_value("author_name", item)?,
        author_id: required_item_value("author_id", item)?,
        title: title,
        content: required_item_value("content", item)?,
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

    pub async fn put_post(&self, post: Post) -> Result<(), DDBError> {
        let mut request = self.client.put_item()
            .table_name(&self.table_name)
            .item("pK", AttributeValue::S(String::from(post.blog_id)))
            .item("sK", AttributeValue::S(String::from(post.post_id)))
            .item("author_name", AttributeValue::S(String::from(post.author_name)))
            .item("author_id", AttributeValue::S(String::from(post.author_id)))
            .item("content", AttributeValue::S(String::from(post.content)));
        
        if let Some(title) = post.title {
            request = request.item("title", AttributeValue::S(String::from(title)));
        }

        match request.send().await {
            Ok(_) => Ok(()),
            Err(_) => Err(DDBError)
        }
    }

    pub async fn put_blog(&self, blog: NewBlog) -> Result<(), DDBError> {
        let mut request = self.client.put_item()
            .table_name(&self.table_name)
            .item("pK", AttributeValue::S(String::from(blog.blog_id)))
            .item("sK", AttributeValue::S(String::from("About")))
            .item("author_name", AttributeValue::S(String::from(blog.author_name)))
            .item("author_id", AttributeValue::S(String::from(blog.author_id)))
            .item("description", AttributeValue::S(String::from(blog.description)))
            .item("style", AttributeValue::S(String::from(blog.style)));

        match request.send().await {
            Ok(_) => Ok(()),
            Err(_) => Err(DDBError)
        }
    }

    pub async fn get_posts(
        &self, 
        blog_id: String, 
        oldest: String,
        newest: String,
    ) -> Vec<Post> {
        error!("GetPosts {} {}",
            oldest.clone(),
            newest.clone()
        );

        let res = self.client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("#blog_id = :blog_id and #post_id between :newest and :oldest")
            .expression_attribute_names("#blog_id", "pK")
            .expression_attribute_names("#post_id", "sK")
            .expression_attribute_values(":blog_id", AttributeValue::S(blog_id))
            .expression_attribute_values(":newest", AttributeValue::S(oldest))
            .expression_attribute_values(":oldest", AttributeValue::S(newest))
            .send()
            .await;

        return match res {
            Ok(output) => {
                output.items
                    .expect("No items in response")
                    .iter()
                    .map(|item| item_to_post(item).expect("Couldn't convert item to post"))
                    .collect()
            },
            Err(error) => {
                error!("{:?}", error);
                vec![]
            }
        }
    }
}