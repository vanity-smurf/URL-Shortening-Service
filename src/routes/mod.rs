use crate::handlers::{create_url, delete_url, get_url, get_url_stats};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/shorten")
            .route("", web::post().to(create_url))
            .route("/{short_code}", web::get().to(get_url))
            .route("/{short_code}", web::delete().to(delete_url))
            .route("/{short_code}/stats", web::get().to(get_url_stats)),
    );
}
