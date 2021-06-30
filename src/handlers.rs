use super::models::{NewUser, User};
use actix_web::{get, post, web, HttpResponse, Responder};
use r2d2;
use r2d2_mysql::MysqlConnectionManager;

type DbPool = r2d2::Pool<MysqlConnectionManager>;

#[get("/users")]
pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Hello from get users!")
}

#[get("/users/{id}/{username}")]
pub async fn get_user_by_id(user: web::Query<User>) -> impl Responder {
    HttpResponse::Ok().body(format!("hello, {}", user.username))
}

#[post("/users")]
pub async fn create_user(pool: web::Data<DbPool>, new_user: web::Json<NewUser>) -> impl Responder {
    let new_user = new_user.into_inner();
    let conn = pool.get().expect("Couldn't connect to database.\n");

    HttpResponse::Ok().body("Hello from create user!")
}
