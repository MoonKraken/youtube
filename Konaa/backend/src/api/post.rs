use crate::repository::ddb::DDBRepository;
use actix_web::{
    post,
    web::Path,
    web::Json,
    web::Data,
};
use common::model::post::{PostIdentifier, NewPost, Post};
use super::error::BlogError;
use chrono::Utc;

#[post("/{blog_id}")]
pub async fn create_post(
    ddb_repo: Data<DDBRepository>,
    blog_id: Path<String>,
    body: Json<NewPost>,
) -> Result<Json<PostIdentifier>, BlogError> {
    let req = body.into_inner();
    let post_id = Utc::now().naive_utc().to_string();
    let blog_id = blog_id.into_inner();
    let new_post = Post {
        blog_id: blog_id.clone(),
        post_id: post_id.clone(),
        author_name: req.author_name,
        author_id: req.author_id,
        title: req.title,
        content: req.content
    };

    let result = ddb_repo.put_post(new_post).await;
    match result {
        Ok(_) => Ok(
            Json(
                PostIdentifier { 
                    blog_id, 
                    post_id,
                }
            )
        ),
        Err(_) => Err(BlogError::PostCreationFailed)
    }
}

#[post("/{blog_id}/{post_id}")]
pub async fn create_comment(
    ddb_repo: Data<DDBRepository>,
    blog_id: Path<String>,
    post_id: Path<String>,
    body: Json<NewPost>,
) -> Result<Json<PostIdentifier>, BlogError> {
    let req = body.into_inner();
    let mut post_id: String = post_id.into_inner();

    post_id.push_str(":");
    post_id.push_str(&Utc::now().naive_utc().to_string());

    let blog_id = blog_id.into_inner();
    let new_post = Post {
        blog_id: blog_id.clone(),
        post_id: post_id.clone(),
        author_name: req.author_name,
        author_id: req.author_id,
        title: req.title,
        content: req.content
    };

    let result = ddb_repo.put_post(new_post).await;
    match result {
        Ok(_) => Ok(
            Json(
                PostIdentifier { 
                    blog_id, 
                    post_id,
                }
            )
        ),
        Err(_) => Err(BlogError::PostCreationFailed)
    }
}