use chrono::{DateTime, Utc};
use diesel::Queryable;
use serde::Serialize;

use crate::{db_models::{Category, Post, User}, derive_with_relations};

derive_with_relations!(
    Post,
    PostWithAuthorCategory {
        id: i64,
        created_at: DateTime<Utc>,
        title: String,
        subtitle: String,
        content: String,
        published: bool,
        author_id: i32,
        category_id: i32,
    },
    {
        author: User,
        category: Category
    }
);

