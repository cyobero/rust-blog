use rust_blog::establish_connection;
use rust_blog::models::User;
use rust_blog::schema::users;

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

fn main() {
    let connection = establish_connection();

    let results: Result<User, _> = users::table.filter(users::id.eq(1)).first(&connection);
    let user = results.unwrap();
    println!("id: {}", user.id);
    println!("username: {}", user.username);
}
