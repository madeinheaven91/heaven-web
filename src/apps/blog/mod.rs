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

    web::scope("/users")
        .route("", web::get().to(handlers::get_posts))
        .route("", web::post().to(handlers::create_post))
        .route("/{id}", web::get().to(handlers::get_post))
        .service(
            web::scope("")
                .wrap(Condition::new(protected, from_fn(auth_middleware)))
                .route("/{id}", web::patch().to(handlers::update_post))
                .route("/{id}", web::delete().to(handlers::delete_post)),
        )
}
