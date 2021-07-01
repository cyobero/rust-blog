use super::schema::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Deserialize, Serialize)]
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

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub author_id: i32,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub author_id: i32,
}
