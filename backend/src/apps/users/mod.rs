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
        .route("", web::get().to(handlers::fetch_users))
        .route("", web::post().to(handlers::new_user))
        .route("/{id}", web::get().to(handlers::fetch_user))
        .route("/login", web::post().to(handlers::login))
        .service(
            web::scope("")
                .wrap(Condition::new(protected, from_fn(auth_middleware)))
                .route("/{id}", web::patch().to(handlers::update_user))
                .route("/{id}", web::delete().to(handlers::delete_user)),
        )
}
