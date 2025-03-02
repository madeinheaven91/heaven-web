use actix_web::{
    middleware::{from_fn, Condition},
    web, Scope,
};

use crate::{middlewares::auth::auth_middleware, shared::statics::CONFIG};

mod forms;
mod handlers;
mod insertables;
mod messages;

pub fn service() -> Scope {
    let protected = CONFIG.environment.auth_enabled();

    web::scope("/blog")
        .service(
            web::scope("/posts")
                .route("", web::get().to(handlers::get_posts))
                .route("/{slug}", web::get().to(handlers::get_post))
                .service(
                    web::scope("")
                        .wrap(Condition::new(protected, from_fn(auth_middleware)))
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
                        .wrap(Condition::new(protected, from_fn(auth_middleware)))
                        .route("", web::post().to(handlers::create_tag))
                        .route("/{slug}", web::patch().to(handlers::update_tag))
                        .route("/{slug}", web::delete().to(handlers::delete_tag)),
                ),
        )
}
