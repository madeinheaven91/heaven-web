#[macro_use]
extern crate diesel;

use actix_web::{
    middleware::{from_fn, Logger},
    web::{self, Data},
    App, HttpServer,
};
use db::{connect, AppState};
use log::info;
use middlewares::{auth::auth_middleware, logging::init_logging};
use shared::statics::{CONFIG, LEXICON};
use std::env;

mod apps;
mod config;
mod db;
mod middlewares;
mod shared;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv::from_filename(env::var("CONFIG").unwrap_or(".env".to_string()));
    let db_addr = connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"));

    init_logging();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/users")
                            .route("", web::get().to(apps::users::handlers::fetch_users))
                            .route("", web::post().to(apps::users::handlers::new_user))
                            .route("/{id}", web::get().to(apps::users::handlers::fetch_user))
                            .route("/login", web::post().to(apps::users::handlers::login))
                            .service(
                                web::scope("")
                                    .route(
                                        "/{id}",
                                        web::patch().to(apps::users::handlers::update_user),
                                    )
                                    .route(
                                        "/{id}",
                                        web::delete().to(apps::users::handlers::delete_user),
                                    )
                                    .wrap(from_fn(auth_middleware)),
                            ),
                    )
                    .service(
                        web::scope("/blog").service(
                            web::scope("/posts")
                                .route("/{id}", web::get().to(apps::blog::handlers::get_post))
                                .route("", web::get().to(apps::blog::handlers::get_posts))
                                .service(
                                    web::scope("")
                                        .route(
                                            "",
                                            web::post().to(apps::blog::handlers::create_post),
                                        )
                                        .route(
                                            "/{id}",
                                            web::patch().to(apps::blog::handlers::update_post),
                                        )
                                        .route(
                                            "/{id}",
                                            web::delete().to(apps::blog::handlers::delete_post),
                                        )
                                        .wrap(from_fn(auth_middleware)),
                                ),
                        ),
                    ),
            )
    })
    .bind((CONFIG.ip.clone(), CONFIG.port))?
    .workers(3)
    .run();

    println!("{}", LEXICON["startup"]);
    info!("Server running at {}:{}", CONFIG.ip, CONFIG.port);

    let res = server.await;

    println!("Shutting down...");

    res
}
