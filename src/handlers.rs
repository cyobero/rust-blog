use super::models::{NewUser, User};
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[get("/users")]
pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Hello from get users!")
}

#[get("/users/{username}")]
pub async fn get_user(pool: web::Data<DbPool>, username: web::Path<String>) -> impl Responder {
    use super::get_user;

    let username = username.into_inner();

    let conn = pool
        .get()
        .expect("Could not establish connection from DB pool.");

    let user: User = web::block(move || get_user(&conn, &username))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    HttpResponse::Ok().json(user)
}

#[post("/users")]
pub async fn create_user(pool: web::Data<DbPool>, new_user: web::Json<NewUser>) -> impl Responder {
    HttpResponse::Ok().body("Hello from create user!")
}
