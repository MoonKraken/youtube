use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct PostIdentifier {
    pub post_id: String,
    pub blog_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    pub blog_id: String,
    pub post_id: String,
    pub author_name: String,
    pub author_id: String,
    pub title: Option<String>,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewPost {
    pub author_name: String,
    pub author_id: String,
    pub title: Option<String>,
    pub content: String
}