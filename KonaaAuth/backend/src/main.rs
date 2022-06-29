mod api;
mod repository;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use api::blog::{
    create_blog,
    get_blog
};
use api::post::{
    create_comment,
    create_post,
};
use api::token::token;
use actix_web_lab::web::spa;

use repository::ddb::DDBRepository;
use actix_web::{HttpServer, App, web::Data, Error, web::scope, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    
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
                    .service(token)
                    .service(create_blog)
                    .service(create_post)
                    .service(create_comment)
                    .service(get_blog)
            )
            .service(
                spa()
                .index_file("./dist/index.html")
                .static_resources_mount("/")
                .static_resources_location("./dist")
                .finish()
            )
    })
    .bind_openssl("0.0.0.0:443", builder)?
    .run()
    .await
}
