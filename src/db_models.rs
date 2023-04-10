#![allow(unused)]
#![allow(clippy::all)]

use chrono::DateTime;
use chrono::offset::Utc;
use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Post {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub title: String,
    pub subtitle: String,
    pub content: String,
    pub published: bool,
    pub author_id: i32,
    pub category_id: i32,
}

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub email: String,
    pub name: String,
}

