use actix_web::middleware::from_fn;
use apistos::web::{self, Scope};

use crate::middlewares::auth::auth_middleware;

mod forms;
mod handlers;
mod insertables;
mod messages;
mod responses;

pub fn service() -> Scope {
    web::scope("/users")
        .route("", web::get().to(handlers::fetch_users))
        .route("", web::post().to(handlers::new_user))
        .route("/login", web::post().to(handlers::login))
        .service(
            web::scope("")
                .wrap(from_fn(auth_middleware))
                .route("/profile", web::get().to(handlers::profile))
                .route("/{id}", web::patch().to(handlers::update_user))
                .route("/{id}", web::delete().to(handlers::delete_user))
        )
        .route("/{id}", web::get().to(handlers::fetch_user))
}
