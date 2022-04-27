use crate::repository::ddb::DDBRepository;
use actix_web::{
    post,
    web::Json,
    web::Data,
};
use common::model::blog::{BlogIdentifier, Blog};
use super::error::BlogError;

#[post("/")]
pub async fn create_blog(
    ddb_repo: Data<DDBRepository>,
    body: Json<Blog>,
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