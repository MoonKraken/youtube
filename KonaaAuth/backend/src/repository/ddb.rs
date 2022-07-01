use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_config::Config;
use common::model::{post::Post};
use common::model::blog::{Blog, NewBlog};
use log::error;
use std::collections::HashMap;

pub struct DDBRepository {
    client: Client,
    table_name: String
}

#[derive(Debug)]
pub struct DDBError;

fn required_item_value(key: &str, item: &HashMap<String, AttributeValue>) -> Result<String, DDBError> {
    match item_value(key, item) {
        Ok(Some(value)) => Ok(value),
        Ok(None) => {
            error!("required field {} was not present", key);
            Err(DDBError)
        },
        Err(DDBError) => Err(DDBError)
    }
}

fn item_value(key: &str, item: &HashMap<String, AttributeValue>) -> Result<Option<String>, DDBError> {
    match item.get(key) {
        Some(value) => match value.as_s() {
            Ok(val) => Ok(Some(val.clone())),
            Err(err) => {
                error!("Error converting field {} to string", key);
                error!("{:?}", err);
                Err(DDBError)
            }
        },
        None => Ok(None)
    }
}

fn items_to_blog(items: Vec<HashMap<String, AttributeValue>>) -> Result<Blog, DDBError> {
    let mut blog = Blog {
        title: None,
        about: None,
        subtitle: None,
        posts: vec![],
    };

    for item in items {
        let sK = required_item_value("sK", &item)?;
        match sK.as_str() {
            "meta" => {
                blog.title = Some(required_item_value("title", &item)?);
                blog.subtitle = Some(required_item_value("subtitle", &item)?);
                blog.about = Some(required_item_value("about", &item)?);
            },
            _ => {
                let title = item_value("title", &item)?;
                blog.posts.push(Post {
                    blog_id: required_item_value("pK", &item)?,
                    post_id: sK,
                    author: required_item_value("author", &item)?,
                    title,
                    content: required_item_value("content", &item)?,
                });
            }
        }
    };

    Ok(blog)
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
            .item("author", AttributeValue::S(String::from(post.author)))
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
        let request = self.client.put_item()
            .table_name(&self.table_name)
            .item("pK", AttributeValue::S(String::from(blog.blog_id)))
            .item("sK", AttributeValue::S(String::from("")))
            .item("title", AttributeValue::S(String::from(blog.title.clone())))
            .item("about", AttributeValue::S(String::from(blog.about.clone())));

        match request.send().await {
            Ok(_) => Ok(()),
            Err(_) => Err(DDBError)
        }
    }

    pub async fn get_blog(
        &self, 
        blog_id: String, 
        oldest: Option<String>,
        newest: Option<String>,
    ) -> Result<Blog, DDBError> {
        let mut res = self.client
            .query()
            .table_name(&self.table_name)
            .expression_attribute_names("#blog_id", "pK")
            .expression_attribute_values(":blog_id", AttributeValue::S(blog_id))
            .key_condition_expression("#blog_id = :blog_id");
            
        if oldest.is_some() || newest.is_some() {
            res = res.expression_attribute_names("#post_id", "sK")
        }

        if let Some(oldest) = oldest {
            res = res.expression_attribute_values(":oldest", AttributeValue::S(oldest))
            .key_condition_expression("#post_id > :oldest")
        }

        if let Some(newest) = newest {
            res = res.expression_attribute_values(":newest", AttributeValue::S(newest))
            .key_condition_expression("#post_id < :newest")
        }

        match res.send().await {
            Ok(res) => {
                let items = res.items.ok_or(DDBError)?;
                items_to_blog(items)
            },
            Err(err) => {
                error!("{}", err.to_string());
                Err(DDBError)
            }
        }
    }
}
