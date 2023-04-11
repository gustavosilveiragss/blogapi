use crate::{
    messages::{CreatePost, FetchPosts, FetchSinglePost, FetchFilteredPosts},
    AppState, DbActor,
};
use actix::Addr;
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use chrono::Utc;

#[get("/posts")]
pub async fn fetch_posts(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchPosts).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No posts found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve posts"),
    }
}

#[get("/category/{id}")]
pub async fn fetch_filtered_posts(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let category_id = path.into_inner();
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchFilteredPosts { category_id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No posts found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve posts"),
    }
}

#[get("/post/{id}")]
pub async fn fetch_single_post(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let post_id = path.into_inner();
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchSinglePost { post_id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No posts found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve posts"),
    }
}

#[post("/post")]
pub async fn create_post(
    state: Data<AppState>,
    body: Json<CreatePost>,
) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db
        .send(CreatePost {
            created_at: Some(Utc::now()),
            title: body.title.to_string(),
            subtitle: body.subtitle.to_string(),
            content: body.content.to_string(),
            published: body.published,
            author_id: body.author_id,
            category_id: body.category_id,
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create post"),
    }
}
