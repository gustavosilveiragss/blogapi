use crate::payload_models::{PostWithAuthorCategory};
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<PostWithAuthorCategory>>")]
pub struct FetchPosts;
