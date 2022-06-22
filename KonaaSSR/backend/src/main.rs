mod api;
mod repository;

use std::fs;
use api::blog::{
    create_blog,
    get_blog
};
use api::post::{
    create_comment,
    create_post,
};

use repository::ddb::DDBRepository;
use actix_web::{HttpServer, App, web::Data, Error, web::scope, middleware::Logger, get, HttpResponse, HttpRequest};
use actix_files as actix_fs;
use yew::ServerRenderer;
use frontend::{ServerApp, App as YewApp, ServerAppProps};

#[get("/{tail:.*}")]
async fn render_yew_app(req: HttpRequest) -> Result<HttpResponse, Error> {
    let index_html_s = fs::read_to_string("./dist/index.html").unwrap();
    let server_app_props = ServerAppProps {
        url: req.uri().to_string().into(),
    };

    let content = ServerRenderer
        ::<ServerApp>
        ::with_props(server_app_props).render().await;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(index_html_s.replace("<body>", &format!("<body>{}", content)))
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = aws_config::load_from_env().await;
    HttpServer::new(move || {
        let ddb_repo: DDBRepository = DDBRepository::init(
            String::from("posts"),
            config.clone()
        );
        let ddb_data = Data::new(
            ddb_repo
        );
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(
                scope("/api")
                    .service(create_blog)
                    .service(create_post)
                    .service(create_comment)
                    .service(get_blog)
            )
            .service(
                actix_fs::Files::new("/dist", "./dist")
            )
            .service(render_yew_app)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
