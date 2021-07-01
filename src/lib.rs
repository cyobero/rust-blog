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

/// Create a new user by passing in username and password and insert new record into database.
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

/// Create new blog post.
pub fn create_post(
    conn: &MysqlConnection,
    title: String,
    body: String,
    author_id: i32,
) -> Result<usize, Error> {
    use schema::posts;

    let new_post = NewPost {
        title,
        body,
        author_id,
    };
    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
}

/// Get post by post id.
pub fn get_post(conn: &MysqlConnection, post_id: i32) -> Result<Post, Error> {
    use schema::posts;

    posts::table.filter(posts::id.eq(post_id)).get_result(conn)
}

/// Get all posts.
pub fn get_posts(conn: &MysqlConnection) -> Result<Vec<Post>, Error> {
    use schema::posts;

    posts::table.get_results(conn)
}

#[cfg(test)]
mod tests {
    #[test]
    fn post_retreived() {
        use super::establish_connection;
        use super::get_post;

        let conn = establish_connection();

        let post1 = get_post(&conn, 1);
        let post2 = get_post(&conn, 2);

        assert_eq!(post1.unwrap().id, 1);
        assert_eq!(post2.unwrap().id, 2);
    }

    #[test]
    fn post_created() {
        use super::create_post;
        use super::establish_connection;

        let conn = establish_connection();

        let title = String::from("This is my favorite song.");
        let body = String::from(
            "You just don't know the words. But I still mess with you; you just
            ain't never heard.",
        );

        // Author new post by user 'cyobero' (user id = 1).
        let post1 = create_post(&conn, title, body, 1);
        // Author new post by user 'ifcyipes' (user id = 17)
        let title = String::from("it's maaahvel baybee!");
        let body =
            String::from("Oh, he go tthe mango Sentinel! This bitch is blue like candy, n***a!");
        let post2 = create_post(&conn, title, body, 17);

        assert_eq!(post1, Ok(1));
        assert_eq!(post2, Ok(1));
    }

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
