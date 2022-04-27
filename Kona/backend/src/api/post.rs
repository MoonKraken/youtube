use crate::repository::ddb::DDBRepository;
use actix_web::{
    get, 
    post,
    web::Path,
    web::Json,
    web::Data,
    web::Query,
};
use serde::Deserialize;
use common::model::post::{PostIdentifier, NewPost, Post};
use super::error::BlogError;
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, Deserialize)]
pub struct DateTimeRange {
    earliest: Option<i64>,
    latest: Option<i64>,
}

fn epoch_to_naive_dt(epoch: i64) -> DateTime<Tz::Utc> {
    DateTime::from_timestamp(epoch, 0)
}

#[get("/{blog_id}")]
pub async fn get_posts(
    ddb_repo: Data<DDBRepository>, 
    blog_id: Path<String>,
    date_range: Query<DateTimeRange>,
) -> Json<Vec<Post>> {
    let inner = date_range.into_inner();

    let earliest = inner.earliest
        .map(epoch_to_naive_dt)
        .expect(DateTime::from_timestamp(0, 0).to_rfc3339());

    let latest = inner.latest
        .map(epoch_to_naive_dt)
        .expect(Utc::now().to_rfc3339());

    let posts = ddb_repo.get_posts(
        blog_id.into_inner(),
        earliest,
        latest,
    ).await;

    Json(posts)
}

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