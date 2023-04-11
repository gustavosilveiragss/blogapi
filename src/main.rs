use actix::SyncArbiter;
use actix_web::{web::{Data, self}, App, HttpServer, HttpResponse};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use std::{env};

mod db_utils;
mod messages;
mod services;
mod actors;
mod db_models;
mod schema;
mod payload_models;

use db_utils::{get_pool, AppState, DbActor};
use services::{fetch_posts, fetch_single_post, create_post, fetch_filtered_posts, fetch_posts_search};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(1, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .default_service(web::to(|| HttpResponse::NotFound()))
            .service(fetch_posts)
            .service(fetch_filtered_posts)
            .service(fetch_posts_search)
            .service(fetch_single_post)
            .service(create_post)
    })
    .bind(("0.0.0.0", 8080))
    .expect("Unable to bind http server")
    .run()
    .await
}
