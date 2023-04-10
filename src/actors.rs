use crate::db_models::{Category, Post, User};
use crate::db_utils::DbActor;
use crate::messages::{CreatePost, FetchPosts, FetchSinglePost};
use crate::payload_models::PostWithAuthorCategory;
use crate::schema::{categories, posts, users};

use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<FetchPosts> for DbActor {
    type Result = QueryResult<Vec<PostWithAuthorCategory>>;

    fn handle(&mut self, _msg: FetchPosts, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch Posts: Unable to establish connection");

        let query = posts::table
            .inner_join(users::table)
            .inner_join(categories::table)
            .select((
                posts::all_columns,
                users::all_columns,
                categories::all_columns,
            ))
            .order_by(posts::created_at.desc());

        let posts_with_author_category = query
            .load::<(Post, User, Category)>(&mut conn)?
            .into_iter()
            .map(|(post, user, category)| PostWithAuthorCategory::build(post, user, category))
            .collect::<Vec<_>>();

        Ok(posts_with_author_category)
    }
}

impl Handler<FetchSinglePost> for DbActor {
    type Result = QueryResult<PostWithAuthorCategory>;

    fn handle(&mut self, msg: FetchSinglePost, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch Post: Unable to establish connection");

        let query = posts::table
            .filter(posts::id.eq(msg.post_id))
            .inner_join(users::table)
            .inner_join(categories::table)
            .select((
                posts::all_columns,
                users::all_columns,
                categories::all_columns,
            ));

        let post_with_author_category = query
            .first::<(Post, User, Category)>(&mut conn)
            .map(|(post, user, category)| PostWithAuthorCategory::build(post, user, category))
            .unwrap();

        Ok(post_with_author_category)
    }
}

impl Handler<CreatePost> for DbActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: CreatePost, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Create Post: Unable to establish connection");

        let new_post = CreatePost {
            created_at: msg.created_at,
            title: msg.title,
            subtitle: msg.subtitle,
            content: msg.content,
            published: msg.published,
            author_id: msg.author_id,
            category_id: msg.category_id,
        };

        diesel::insert_into(posts::table)
            .values(new_post)
            .returning(posts::all_columns)
            .get_result::<Post>(&mut conn)
    }
}
