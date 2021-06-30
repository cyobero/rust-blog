use super::schema::*;

use diesel::sql_types::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

/// Create new user.
#[derive(Debug, Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: Timestamp,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
