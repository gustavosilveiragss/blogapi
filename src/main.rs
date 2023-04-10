use actix::SyncArbiter;
use actix_web::{web::Data, App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use std::env;

mod db_utils;
mod messages;
mod services;
mod actors;
mod db_models;
mod schema;
mod payload_models;
// mod insertables;

use db_utils::{get_pool, AppState, DbActor};
use services::fetch_posts;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);

    // not fully understanding this one tbh, oh well
    let db_addr = SyncArbiter::start(4, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .service(fetch_posts)
    })
    .bind(("127.0.0.1", 8080))
    .expect("Unable to bind http server")
    .run()
    .await
}
