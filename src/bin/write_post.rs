use rust_blog::{create_post, establish_connection};

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

fn main() {
    let connection = establish_connection();
}
