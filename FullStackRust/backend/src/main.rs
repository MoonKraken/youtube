mod api;
mod repository;

use api::task::{
    get_task,
    submit_task,
    start_task,
    complete_task,
    pause_task,
    fail_task,
};
use repository::ddb::DDBRepository;
use actix_web::{HttpServer, App, web::Data, web::scope, web, middleware::Logger};
use actix_files::{Files, NamedFile};

async fn index_file() -> actix_files::NamedFile {
    NamedFile::open("./dist/index.html").unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = aws_config::load_from_env().await;
    HttpServer::new(move || {
        let ddb_repo: DDBRepository = DDBRepository::init(
            String::from("task"),
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
                    .service(get_task)
                    .service(submit_task)
                    .service(start_task)
                    .service(complete_task)
                    .service(pause_task)
                    .service(fail_task)
            )
            .service(
                Files::new("/", "./dist")
                    .index_file("index.html")
            )
            .default_service(web::get().to(index_file))

    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
