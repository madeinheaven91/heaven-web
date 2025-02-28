#[macro_use]
extern crate diesel;

use actix::SyncArbiter;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use config::Config;
use db::pg::{get_pool, AppState, DbActor};
use std::env;

mod apps;
mod config;
mod db;
mod shared;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // fn main() -> std::io::Result<()> {
    let config = Config::init(env::var("CONFIG").unwrap_or(".env".to_string()).as_str());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = get_pool(&database_url);
    let db_addr = SyncArbiter::start(5, move || DbActor { pool: pool.clone() });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/users")
                            .route("", web::post().to(apps::users::handlers::new_user))
                            .route("", web::get().to(apps::users::handlers::fetch_users))
                            .route("/{id}", web::get().to(apps::users::handlers::fetch_user))
                            .route("/{id}", web::patch().to(apps::users::handlers::update_user))
                            .route( "/{id}", web::delete().to(apps::users::handlers::delete_user),),
                    )
                    .service(
                        web::scope("/blog").service(
                            web::scope("/posts")
                                .route("", web::post().to(apps::blog::handlers::create_post))
                                .route("", web::get().to(apps::blog::handlers::get_posts))
                                .route("/{id}", web::get().to(apps::blog::handlers::get_post))
                                .route("/{id}", web::patch().to(apps::blog::handlers::update_post))
                                .route( "/{id}", web::delete().to(apps::blog::handlers::delete_post),),
                        ),
                    ),
            )
    })
    .bind((config.server.ip.clone(), config.server.port))?
    .workers(3)
    .run();

    println!(
        "Server running at {}:{}",
        config.server.ip, config.server.port
    );

    let res = server.await;

    println!("Shutting down...");

    res
}
