use serde::{Serialize, Deserialize};
use super::post::Post;

#[derive(Serialize)]
pub struct BlogIdentifier {
    pub blog_id: String,
}

#[derive(Deserialize)]
pub struct NewBlog {
    pub blog_id: String,
    pub title: String,
    pub owner: String,
    pub description: String,
    pub style: String,
}

#[derive(Deserialize)]
pub struct Blog {
    pub title: String,
    pub about: String,
    pub posts: Vec<Post>,
}