use actix::SyncArbiter;
use actix_cors::Cors;
use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use std::env;

mod actors;
mod db_models;
mod messages;
mod payload_models;
mod schema;
mod services;
mod utils;

use services::{
    create_post, fetch_filtered_posts, fetch_posts, fetch_posts_search, fetch_single_post, fetch_categories,
};
use utils::{get_pool, AppState, DbActor};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://blog.gustavosilveira.codes")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .wrap(cors)
            .default_service(web::to(|| HttpResponse::NotFound()))
            .service(fetch_posts)
            .service(fetch_categories)
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
