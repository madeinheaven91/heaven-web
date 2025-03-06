use actix_web::middleware::from_fn;
use apistos::web::{self, Scope};

use crate::middlewares::auth::{access_mw, admin_access_mw, refresh_mw};

mod forms;
mod handlers;
mod insertables;
pub mod messages;
pub mod responses;

pub fn service() -> Scope {
    web::scope("/users")
        .route("/fetch", web::get().to(handlers::fetch_users))
        .route("/fetch/{id}", web::get().to(handlers::fetch_user))
        .route("/login", web::post().to(handlers::login))
        .service(
            web::scope("/new")
                .wrap(from_fn(admin_access_mw))
                .route("", web::post().to(handlers::new_user))
        )
        .service(
            web::scope("")
                .wrap(from_fn(access_mw))
                .route("/logout", web::get().to(handlers::logout))
                .route("/profile", web::get().to(handlers::profile))
                .route("/{id}", web::patch().to(handlers::update_user))
                .route("/{id}", web::delete().to(handlers::delete_user))
        )
        .service(
            web::scope("/refresh")
                .wrap(from_fn(refresh_mw))
                .route("", web::get().to(handlers::refresh_token))
        )
}
