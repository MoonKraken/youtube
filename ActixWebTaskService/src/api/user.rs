use actix_web::{get, post, put, delete, Responder, HttpResponse};

#[get("/user")]
pub async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("get_user")
}

#[post("/user")]
pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("create_user")
}

#[put("/user")]
pub async fn update_user() -> impl Responder {
    HttpResponse::Ok().body("update_user")
}

#[delete("/user")]
pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("delete_user")
}