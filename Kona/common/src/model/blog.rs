use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct BlogIdentifier {
    pub blog_id: String,
}

#[derive(Deserialize)]
pub struct Blog {
    pub blog_id: String,
    pub title: String,
    pub author_name: String,
    pub author_id: String,
    pub description: String,
    pub style: String,
}