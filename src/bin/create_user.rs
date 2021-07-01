use rust_blog::create_user;
use rust_blog::establish_connection;
use rust_blog::models::User;
use rust_blog::schema::users;

use clap::{App, Arg};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

fn main() {
    let connection = establish_connection();

    let matches = App::new("Create new user")
        .version("1.0")
        .about("Creates new user and inserts into database for `rust_blog` app.")
        .author("Czar Yobero")
        .arg(
            Arg::with_name("username")
                .short("u")
                .long("username")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .index(2)
                .required(true),
        )
        .get_matches();

    let username = matches.value_of("username").expect("Username not set.");
    let password = matches.value_of("password").expect("Password not set.");

    println!("Creating user...\n");

    create_user(&connection, username.to_string(), password.to_string());

    let user: User = users::table
        .filter(users::username.eq(username))
        .get_result(&connection)
        .unwrap();

    print!("Created user: ");
    println!("{:?}", user);
}
