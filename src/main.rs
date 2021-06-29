use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use r2d2::Pool;
use r2d2_mysql::mysql::{Opts, OptsBuilder};
use r2d2_mysql::MysqlConnectionManager;
use std::env;
use std::sync::Arc;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello there, ya filthy animal!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL").unwrap();
    let opts = Opts::from_url(&database_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MysqlConnectionManager::new(builder);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.\n");

    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
