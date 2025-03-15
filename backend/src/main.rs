#[macro_use]
extern crate diesel;

#[allow(unused_imports)]

use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use apistos::{
    app::{BuildConfig, OpenApiWrapper}, info::Info, server::Server, spec::Spec, SwaggerUIConfig
};
use db::{AppState, connect};
use log::{debug, info, LevelFilter};
use middlewares::logging::{init_logging, log_requests_middleware};
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
                    title: "Heaven web API".to_string(),
                    description: None,
                    ..Default::default()
                },
                servers: vec![Server {
                    url: "/".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            };
            let build_config = match CONFIG.environment.dev() {
                true => BuildConfig::default().with(SwaggerUIConfig::new(&"/swagger")),
                false => BuildConfig::default(),
            };
            let cors = Cors::default()
                .allow_any_method()
                .allowed_origin_fn(|header, _| {
                    header.to_str().unwrap().starts_with("http://localhost")
                })
                .allowed_origin("https://madeinheaven.space")
                .allow_any_header()
                .supports_credentials()
                .max_age(3600)
                .expose_any_header();

            App::new()
                .document(spec)
                .app_data(Data::new(AppState {
                    db: db_addr.clone(),
                }))
                .wrap(Logger::new(
                    "%a | \"%r\" %s %b bytes \"%{Referer}i\" \"%{User-Agent}i\" Handled in %D ms",
                ))
                .wrap(cors)
                // .wrap(from_fn(log_requests_middleware))
                .service(
                    apistos::web::scope("/api/v1")
                        .service(apps::users::service())
                        .service(apps::blog::service()),
                )
                .build_with("/openapi.json", build_config)
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
            let cors = Cors::default()
                .allowed_origin_fn(|origin, _req| {
                    origin.to_str().unwrap().starts_with("http://localhost")
                })
                .allow_any_origin()
                .allow_any_header()
                .expose_any_header()
                .allow_any_method()
                .supports_credentials()
                .max_age(3600)
            ;
    debug!("Cors: {:?}", cors);

    let res = server.await;

    info!("Shutting down...");

    res
}
