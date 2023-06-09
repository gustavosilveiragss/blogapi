use crate::payload_models::{PostWithAuthorCategory, PostWithCategory};
use crate::db_models::{Post, Category};
use crate::schema::posts;
use actix::Message;
use chrono::{DateTime, Utc};
use diesel::{QueryResult, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<PostWithCategory>>")]
pub struct FetchPosts;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Category>>")]
pub struct FetchCategories;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<PostWithCategory>>")]
pub struct FetchFilteredPosts {
    pub category_ids: Vec<i32>
}


#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Post>>")]
pub struct FetchPostsSearch {
    pub title: String
}

#[derive(Message)]
#[rtype(result = "QueryResult<PostWithAuthorCategory>")]
pub struct FetchSinglePost {
    pub post_id: i64
}

#[derive(Message, Deserialize, Serialize, Insertable, Clone)]
#[rtype(result = "QueryResult<Post>")]
#[diesel(table_name=posts)]
pub struct CreatePost {
    pub created_at: Option<DateTime<Utc>>,
    pub title: String,
    pub subtitle: String,
    pub content: String,
    pub published: bool,
    pub author_id: i32,
    pub category_id: i32,
}
