#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::Data,
    App, HttpServer,
};
use apistos::{app::{BuildConfig, OpenApiWrapper}, info::Info, server::Server, spec::Spec, SwaggerUIConfig};
use db::{connect, AppState};
use log::{info, LevelFilter};
use middlewares::logging::init_logging;
use shared::statics::{CONFIG, LEXICON};

mod apps;
mod config;
mod db;
mod middlewares;
mod shared;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let _ = dotenv::dotenv();
    let db_addr = connect(CONFIG.db_url.as_str());
    init_logging();

    let server = HttpServer::new(move || {
        let spec = Spec {
            info: Info {
                title: "A well documented API".to_string(),
                description: Some(
                "This is an API documented using Apistos,\na wonderful new tool to document your actix API !".to_string(),
                ),
                ..Default::default()
            },
            servers: vec![Server {
                // url: "/api/v1".to_string(),
                url: "/".to_string(),
                ..Default::default()
            }],
            ..Default::default()
            };
        App::new()
            .document(spec)
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .wrap(Logger::new(
                "%a | \"%r\" %s %b bytes \"%{Referer}i\" \"%{User-Agent}i\" Handled in %D ms",
            ))
            .wrap(Cors::default()
                .allowed_origin("http://localhost:8080"))
            .service(
                apistos::web::scope("/api/v1")
                    .service(apps::users::service())
                    .service(apps::blog::service()),
            )
            .build_with("/openapi.json",
                BuildConfig::default()
                    .with(SwaggerUIConfig::new(&"/swagger")
            ))
    })
    .bind(("0.0.0.0", CONFIG.port))?
    .workers(3)
    .run();

    info!("{}", LEXICON["startup"]);
    info!("Server running at 0.0.0.0:{}", CONFIG.port);
    info!("Log level: {}", CONFIG.log_level);
    if !matches!(CONFIG.log_level, LevelFilter::Off) {
        info!("Log file: {}.log", CONFIG.log_file);
    }
    info!("Environment: {}", format!("{:?}", CONFIG.environment));

    let res = server.await;

    info!("Shutting down...");

    res
}
