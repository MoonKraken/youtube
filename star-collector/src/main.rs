use actix_web::{
    error::{ErrorNotFound, ErrorInternalServerError},
    get,
    web::{Path, ServiceConfig},
    Result,
};
use octocrab::{models::Repository, Page};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/stars/{user}")]
async fn stars(user: Path<String>) -> Result<String> {
    let octocrab = octocrab::instance();
    let user_str: String = user.into_inner();
    let mut page: Page<Repository> = octocrab
        .users(user_str)
        .repos()
        .per_page(50)
        .send()
        .await
        .map_err(|_| {
                ErrorNotFound(
                    "issue looking up user stars"
                )
            }
        )?;

    let mut accum = 0;
    loop {
        for repo in &page {
            accum = accum + repo
                .stargazers_count
                .unwrap_or(0);
        }

        page = match octocrab
            .get_page::<Repository>(&page.next)
            .await
            .map_err(|_| {
                    ErrorInternalServerError(
                        "issue looking up user stars"
                    )
                }
            )?
        {
            Some(next_page) => next_page,
            None => break,
        }
    }

    Ok(accum.to_string())
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(stars);
    };

    Ok(config.into())
}
