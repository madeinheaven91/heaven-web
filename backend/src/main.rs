#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use apistos::{
    SwaggerUIConfig,
    app::{BuildConfig, OpenApiWrapper},
    info::Info,
    server::Server,
    spec::Spec,
};
use db::{AppState, connect};
use log::{LevelFilter, info};
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

    let server = if matches!(CONFIG.environment, config::Environment::Production) {
        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(AppState {
                    db: db_addr.clone(),
                }))
                .wrap(Logger::new(
                    "%a | \"%r\" %s %b bytes \"%{Referer}i\" \"%{User-Agent}i\" Handled in %D ms",
                ))
                .wrap(
                    Cors::default().allowed_origin(
                        format!("http://localhost:{}", CONFIG.frontend_port).as_str(),
                    ),
                )
                .service(
                    apistos::web::scope("/api/v1")
                        .service(apps::users::service())
                        .service(apps::blog::service()),
                )
        })
        .bind(("0.0.0.0", CONFIG.port))?
        .workers(3)
        .run()
    } else {
        HttpServer::new(move || {
            let spec = Spec {
                info: Info {
                    title: "Heaven web API".to_string(),
                    description: None,
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
                .wrap(
                    Cors::default().allowed_origin(
                        format!("http://localhost:{}", CONFIG.frontend_port).as_str(),
                    ),
                )
                .service(
                    apistos::web::scope("/api/v1")
                        .service(apps::users::service())
                        .service(apps::blog::service()),
                )
                .build_with(
                    "/openapi.json",
                    BuildConfig::default().with(SwaggerUIConfig::new(&"/swagger")),
                )
        })
        .bind(("0.0.0.0", CONFIG.port))?
        .workers(3)
        .run()
    };

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
