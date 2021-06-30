// lib.rs
#[macro_use]
extern crate diesel;

use models::NewUser;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use std::env;

pub mod handlers;
pub mod models;
pub mod schema;

pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.\n");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &MysqlConnection, _username: String, _password: String) {
    use schema::users::dsl::*;

    let new_user = NewUser {
        username: _username,
        password: _password,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user.\n");
}

#[cfg(test)]
mod tests {
    #[test]
    fn user_created() {
        use super::create_user;
        use super::establish_connection;

        let connection = establish_connection();

        let _username = String::from("cyobero");
        let _password = String::from("password");

        create_user(&connection, _username, _password);
    }
}
