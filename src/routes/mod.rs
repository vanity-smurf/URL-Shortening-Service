use actix_web::web;
use crate::handlers::{create_url, get_url};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/create_url", web::post().to(create_url))
            .route("/get_url/{short_code}", web::get().to(get_url))
    );
}
