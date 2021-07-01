use super::models::NewUser;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use serde::{Deserialize, Serialize};

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    id: i32,
    username: String,
}

#[get("/users")]
pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Hello from get users!")
}

#[get("/users/{username}")]
pub async fn get_user(
    pool: web::Data<DbPool>,
    username: web::Path<String>,
) -> Result<HttpResponse> {
    use super::get_user;

    let username = username.into_inner();

    let conn = pool
        .get()
        .expect("Could not establish connection from DB pool.");

    let user = web::block(move || get_user(&conn, &username))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(UserResponse {
        id: user.id,
        username: user.username,
    }))
}

#[post("/users")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>,
) -> Result<HttpResponse> {
    use super::create_user;

    let username = new_user.username.to_string();
    let password = new_user.password.to_string();

    let conn = pool
        .get()
        .expect("Could not establish connection from DB pool.");

    let new_user = web::block(move || create_user(&conn, username, password))
        .await
        .map_err(|e| {
            eprint!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    match new_user {
        1 => Ok(HttpResponse::Ok().body("New user successfully created!")),
        _ => Ok(HttpResponse::Ok().body("whoops. something went wrong")),
    }
}
