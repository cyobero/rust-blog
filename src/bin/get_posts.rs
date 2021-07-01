use rust_blog::establish_connection;
use rust_blog::models::Post;
use rust_blog::schema::posts;

use diesel::RunQueryDsl;

fn main() {
    let connection = establish_connection();

    let results: Vec<Post> = posts::table.get_results(&connection).unwrap();

    results.into_iter().for_each(|post| println!("{:?}", post));
}
