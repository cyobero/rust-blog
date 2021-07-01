use rust_blog::establish_connection;
use rust_blog::models::User;
use rust_blog::schema::users;

use diesel::RunQueryDsl;

fn main() {
    let connection = establish_connection();

    let results: Vec<User> = users::table.get_results(&connection).unwrap();

    results.into_iter().for_each(|user| println!("{:?}", user));
}
