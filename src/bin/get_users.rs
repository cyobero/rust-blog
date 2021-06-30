use rust_blog::establish_connection;
use rust_blog::models::User;
use rust_blog::schema::users::dsl::*;

use diesel::sql_query;
use diesel::sql_types::Integer;

fn main() {
    let connection = establish_connection();
}
