use actix_web::middleware::from_fn;

use crate::middlewares::auth::{access_mw, admin_access_mw};
use apistos::web::{self, Scope};

mod forms;
mod handlers;
mod insertables;
mod messages;
mod responses;
#[cfg(test)]
mod test;

pub fn service() -> Scope {
    web::scope("/blog")
        .service(
            web::scope("/tags")
                .route("/fetch", web::get().to(handlers::get_tags))
                .route("/fetch/{slug}", web::get().to(handlers::get_tag))
                .service(
                    web::scope("")
                        .wrap(from_fn(admin_access_mw))
                        .route("/new", web::post().to(handlers::create_tag))
                        .route("/{slug}", web::patch().to(handlers::update_tag))
                        .route("/{slug}", web::delete().to(handlers::delete_tag)),
                )
        )
        .service(
            web::scope("/posts")
                .route("/fetch", web::get().to(handlers::get_posts))
                .route("/fetch/{slug}", web::get().to(handlers::get_post))
                .service(
                    web::scope("")
                        .wrap(from_fn(access_mw))
                        .route("/new", web::post().to(handlers::create_post))
                        .route("/{post_slug}/assign/{tag_slug}", web::post().to(handlers::assign_tag_to_post))
                        .route("/{post_slug}/withdraw/{tag_slug}", web::delete().to(handlers::rm_tag_from_post))
                        .route("/{slug}", web::patch().to(handlers::update_post))
                        .route("/{slug}", web::delete().to(handlers::delete_post)),
                )
        )
}

