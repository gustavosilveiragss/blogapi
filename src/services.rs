use crate::{messages::FetchPosts, AppState, DbActor};
use actix::Addr;
use actix_web::{get, web::Data, HttpResponse, Responder};

#[get("/posts")]
pub async fn fetch_posts(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchPosts).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No posts found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve posts"),
    }
}
