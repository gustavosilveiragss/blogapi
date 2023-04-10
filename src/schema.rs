// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    posts (id) {
        id -> Int8,
        created_at -> Timestamptz,
        title -> Text,
        subtitle -> Text,
        content -> Text,
        published -> Bool,
        author_id -> Int4,
        category_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        created_at -> Timestamptz,
        email -> Text,
        name -> Text,
    }
}

diesel::joinable!(posts -> categories (category_id));
diesel::joinable!(posts -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    posts,
    users,
);
