table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Nullable<Text>,
        created_at -> Timestamp,
        author_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(posts -> users (author_id));

allow_tables_to_appear_in_same_query!(posts, users,);
