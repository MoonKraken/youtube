mod api;
mod model;
mod repository;

use api::task::{
    get_task,
    submit_task,
    start_task,
    complete_task,
    pause_task,
    fail_task,
};
use repository::db::DB;
use actix_web::{HttpServer, App, web::Data, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let ddb_repo: DB = DB::init();
        let ddb_data = Data::new(
            ddb_repo
        );
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(get_task)
            .service(submit_task)
            .service(start_task)
            .service(complete_task)
            .service(pause_task)
            .service(fail_task)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
