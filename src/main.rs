use rust_blog::handlers::*;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use dotenv::dotenv;
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello there, ya filthy animal!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.\n");

    println!("Starting server at port 8080...");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(index)
            .service(get_users)
            .service(get_user)
            .service(create_user)
            .service(get_post)
            .service(create_post)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
