table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        created_at -> Timestamp,
        author_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(posts -> users (author_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
