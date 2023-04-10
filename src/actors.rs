use crate::db_models::{Post, User, Category};
use crate::db_utils::DbActor;
use crate::messages::FetchPosts;
use crate::payload_models::PostWithAuthorCategory;
use crate::schema::{posts, users};
use actix::Handler;
use chrono::DateTime;
use diesel::{self, prelude::*};

impl Handler<FetchPosts> for DbActor {
    type Result = QueryResult<Vec<PostWithAuthorCategory>>;

    fn handle(&mut self, _msg: FetchPosts, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch Post: Unable to establish connection");
        
        let posts_with_author = posts::table
            .inner_join(users::table)
            .select((posts::all_columns, users::all_columns))
            .order_by(posts::created_at.desc())
            .load::<(Post, User)>(&mut conn)?
            .into_iter()
            .map(|(post, author)| PostWithAuthorCategory::from((
                post,
                author,
                Category { id: 0, name: Some(String::from("a")), created_at: DateTime::default() }
            )))
            .collect();

        Ok(posts_with_author)
    }
}
