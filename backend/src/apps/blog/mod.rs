use actix_web::middleware::from_fn;

use crate::middlewares::auth::auth_middleware;
use apistos::web::{self, Scope};

mod forms;
mod handlers;
mod insertables;
mod messages;
mod responses;

pub fn service() -> Scope {
    web::scope("/blog")
        .service(
            web::scope("/posts")
                .route("", web::get().to(handlers::get_posts))
                .route("/{slug}", web::get().to(handlers::get_post))
                .service(
                    web::scope("")
                        .wrap(from_fn(auth_middleware))
                        .route("", web::post().to(handlers::create_post))
                        .route("/{slug}", web::patch().to(handlers::update_post))
                        .route("/{slug}", web::delete().to(handlers::delete_post)),
                ),
        )
        .service(
            web::scope("/tags")
                .route("", web::get().to(handlers::get_tags))
                .route("/{slug}", web::get().to(handlers::get_tag))
                .service(
                    web::scope("")
                        .wrap(from_fn(auth_middleware))
                        .route("", web::post().to(handlers::create_tag))
                        .route("/{slug}", web::patch().to(handlers::update_tag))
                        .route("/{slug}", web::delete().to(handlers::delete_tag)),
                ),
        )
}
