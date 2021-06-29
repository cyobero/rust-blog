use actix_web::{get, post, HttpResponse, Responder};

#[get("/users")]
pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Hello from get users!")
}

#[get("/users/{id}")]
pub async fn get_user_by_id() -> impl Responder {
    HttpResponse::Ok().body("Hello from get user by id!")
}

#[post("/users")]
pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("Hello from create user!")
}
