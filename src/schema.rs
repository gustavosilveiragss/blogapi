// @generated automatically by Diesel CLI.

diesel::table! {
    Category (id) {
        id -> Int4,
        name -> Nullable<Text>,
        createdAt -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    Post (id) {
        id -> Int8,
        createdAt -> Timestamptz,
        title -> Text,
        subtitle -> Text,
        content -> Text,
        published -> Bool,
        authorId -> Int4,
        categoryId -> Int4,
    }
}

diesel::table! {
    User (id) {
        id -> Int4,
        createdAt -> Timestamptz,
        email -> Text,
        name -> Text,
    }
}

diesel::joinable!(Post -> Category (categoryId));
diesel::joinable!(Post -> User (authorId));

diesel::allow_tables_to_appear_in_same_query!(
    Category,
    Post,
    User,
);
