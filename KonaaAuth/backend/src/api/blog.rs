use crate::repository::ddb::DDBRepository;
use jsonwebtokens_cognito::KeySet;
use actix_web::{
    post,
    get,
    web::{
        Json,
        Data,
        Query,
        Path,
    },
    HttpRequest,
};
use log::error;
use chrono::{DateTime};
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

fn validate_dt(
    dt: Option<String>,
) -> Result<(), BlogError> {
    match dt {
        Some(dt) => {
            let datetime = DateTime::parse_from_rfc3339(dt.as_str());
            match datetime {
                Ok(_) => Ok(()),
                Err(_) => Err(BlogError::DateTimeParseError)
            }
        },
        None => Ok(())
    }
}

#[get("/{blog_id}")]
pub async fn get_blog(
    ddb_repo: Data<DDBRepository>, 
    blog_id: Path<String>,
    date_range: Query<DateTimeRange>,
    request: HttpRequest,
) -> Result<Json<Blog>, BlogError> {
    let keyset = KeySet::new("us-west-2", "us-west-2_7XdFXdQUm")
        .expect("Issue creating keyset");
    let verifier = keyset.new_access_token_verifier(&["604tk757p8f5b61m4n7od2fj48"]).build().expect("Issue with verification");
    
    let auth_header: &str = request
        .headers()
        .get("Authorization")
        .expect("No auth header")
        .to_str()
        .expect("Error converting to str");

    let verified = keyset.verify(auth_header, &verifier).await;
    if let Ok(val) = verified {
        error!("blabhalsfowijefojwef");
        error!("{:?}", val);
    } else {
        return Err(BlogError::Unauthorized);
    }
    error!("asoijwefoijef");

    let inner = date_range.into_inner();
    validate_dt(inner.earliest.clone())?;
    validate_dt(inner.latest.clone())?;

    let blog = ddb_repo.get_blog(
        blog_id.into_inner(),
        inner.earliest,
        inner.latest,
    )
    .await
    .map_err(|_| {
        BlogError::BlogNotFound
    })?;
    Ok(Json(blog))
}
