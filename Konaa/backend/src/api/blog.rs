use crate::repository::ddb::DDBRepository;
use actix_web::{
    post,
    get,
    web::Json,
    web::Data,
    web::Path,
    web::Data,
    web::Query,
};
use chrono::{DateTime, FixedOffset};
use chrono::Utc;
use crate::api::error::BlogError::DateTimeParseError;
use serde::Deserialize;
use common::model::blog::{BlogIdentifier, NewBlog, Blog};
use super::error::BlogError;

#[post("/")]
pub async fn create_blog(
    ddb_repo: Data<DDBRepository>,
    body: Json<NewBlog>,
) -> Result<Json<BlogIdentifier>, BlogError> {
    let req = body.into_inner();
    let blog_id = req.blog_id.clone();

    let result = ddb_repo.put_blog(req).await;
    match result {
        Ok(_) => Ok(
            Json(
                BlogIdentifier { 
                    blog_id,
                }
            )
        ),
        Err(_) => Err(BlogError::PostCreationFailed)
    }
}

#[derive(Debug, Deserialize)]
pub struct DateTimeRange {
    earliest: Option<String>,
    latest: Option<String>,
}

fn validate_or_default_dt(dt: Option<String>, default: DateTime<FixedOffset>) -> Result<String, BlogError> {
    let res = dt
        .map(|datetime| DateTime::parse_from_rfc3339(datetime.as_str()))
        .unwrap_or(Ok(default))
        .map_err(|_| DateTimeParseError)?
        .to_rfc3339();

    Ok(res)
}

#[get("/{blog_id}")]
pub async fn get_blog(
    ddb_repo: Data<DDBRepository>, 
    blog_id: Path<String>,
    date_range: Query<DateTimeRange>,
) -> Result<Blog, BlogError> {
    let inner = date_range.into_inner();

    let earliest = validate_or_default_dt(
        inner.earliest,
        DateTime::from(Utc.timestamp(0, 0))
    )?;

    let latest = validate_or_default_dt(
        inner.latest,
        DateTime::from(Utc::now())
    )?;

    let posts = ddb_repo.get_posts(
        blog_id.into_inner(),
        earliest,
        latest,
    ).await;

    Ok(Json(posts))
}