// lib.rs
#[macro_use]
extern crate diesel;

use models::*;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use std::env;

pub mod handlers;
pub mod models;
pub mod schema;

/// Establish a connection to MySQL database.
pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.\n");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/// Create a new user and insert new record into database.
pub fn create_user(
    conn: &MysqlConnection,
    _username: String,
    _password: String,
) -> Result<usize, Error> {
    use schema::users::dsl::*;

    let new_user = NewUser {
        username: _username,
        password: _password,
    };

    diesel::insert_into(users).values(&new_user).execute(conn)
}

/// Retrieves `User` object by username.
pub fn get_user(conn: &MysqlConnection, username: &str) -> Result<User, Error> {
    use schema::users;

    users::table
        .filter(users::username.eq(username.to_string()))
        .get_result(conn)
}

#[cfg(test)]
mod tests {
    #[test]
    fn user_retreived() {
        use super::establish_connection;
        use super::get_user;

        let conn = establish_connection();
        let czar = get_user(&conn, "cyobero").unwrap();
        let homer = get_user(&conn, "homerS69").unwrap();

        assert_eq!(czar.username, "cyobero");
        assert_eq!(homer.username, "homerS69");
    }

    #[test]
    fn user_created() {
        use super::create_user;
        use super::establish_connection;

        let connection = establish_connection();

        let _username = String::from("homerS69");
        let _password = String::from("password123");

        let new_user = create_user(&connection, _username, _password);

        // Since we already created this user.
        assert_ne!(new_user, Ok(1));
    }
}
