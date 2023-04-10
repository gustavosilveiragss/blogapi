use crate::db_models::{Category, Post, User};
use crate::db_utils::DbActor;
use crate::messages::{FetchPosts};
use crate::payload_models::{PostWithAuthorCategory};
use crate::schema::{categories, posts, users};
use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<FetchPosts> for DbActor {
    type Result = QueryResult<Vec<PostWithAuthorCategory>>;

    fn handle(&mut self, _msg: FetchPosts, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch Post: Unable to establish connection");

        let query = posts::table
            .inner_join(users::table)
            .inner_join(categories::table)
            .select((posts::all_columns, users::all_columns, categories::all_columns))
            .order_by(posts::created_at.desc());

        let posts_with_author_category = query
            .load::<(Post, User, Category)>(&mut conn)?
            .into_iter()
            .map(|(post, user, category)| PostWithAuthorCategory::build(post, user, category))
            .collect::<Vec<_>>();

        Ok(posts_with_author_category)
    }
}