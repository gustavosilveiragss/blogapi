use chrono::{DateTime, Utc};
use diesel::Queryable;
use serde::Serialize;

use crate::db_models::{Post, User, Category};

#[derive(Queryable, Debug, Serialize)]
pub struct PostWithAuthorCategory {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub title: String,
    pub subtitle: String,
    pub content: String,
    pub published: bool,
    pub author: User,
    pub category: Category,
}

impl From<(Post, User, Category)> for PostWithAuthorCategory {
    fn from((post, author, category): (Post, User, Category)) -> Self {
        Self {
            id: post.id,
            title: post.title,
            created_at: post.created_at,
            subtitle: post.subtitle,
            content: post.content,
            published: post.published,
            
            author,
            category
        }
    }
}