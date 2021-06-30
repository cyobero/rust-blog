use super::models::User;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/users")]
pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Hello from get users!")
}

#[get("/users/{id}/{username}")]
pub async fn get_user_by_id(user: web::Query<User>) -> impl Responder {
    HttpResponse::Ok().body(format!("hello, {}", user.username))
}

#[post("/users")]
pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("Hello from create user!")
}
