use actix_web::{web, HttpResponse};
use crate::{models, redis::RedisPool};
use chrono::Utc;
use serde_json;
use uuid::Uuid;
use redis_r2d2::redis::Commands;

pub async fn create_url(
    pool: web::Data<RedisPool>,
    payload: web::Json<models::NewURL>
) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    let short_code = Uuid::new_v4().to_string();
    println!("{}", short_code);

    let shortened_url = models::ShortenedURL {
        id: 0,
        url: payload.url.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        clicks: 0,
    };

    let serialized_url = serde_json::to_string(&shortened_url).unwrap();
    let _: () = conn.set(&short_code, serialized_url).unwrap();

    HttpResponse::Ok().json(shortened_url)
}

pub async fn get_url(
    pool: web::Data<RedisPool>,
    short_code: web::Path<String>,
) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    let serialized_url: Option<String> = conn.get(&*short_code).unwrap();

    if let Some(serialized_url) = serialized_url {
        let shortened_url: models::ShortenedURL = serde_json::from_str(&serialized_url).unwrap();
        HttpResponse::Ok().json(shortened_url)
    } else {
        HttpResponse::NotFound().body("URL not found")
    }
}

