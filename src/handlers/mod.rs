use crate::{models, redis::RedisPool};
use actix_web::{HttpResponse, web};
use chrono::Utc;
use redis_r2d2::redis::Commands;
use serde_json;
use uuid::Uuid;

pub async fn create_url(
    pool: web::Data<RedisPool>,
    payload: web::Json<models::NewURL>,
) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    let short_code = Uuid::new_v4().to_string();

    let shortened_url = models::ShortenedURL {
        id: 0,
        url: payload.url.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        access_count: 0,
    };

    let serialized_url = serde_json::to_string(&shortened_url).unwrap();
    let _: () = conn.set(&short_code, serialized_url).unwrap();

    let base_url = "https://domain.com"; // TODO it nia .env
    let full_short_url = format!("{}/{}", base_url, short_code);

    HttpResponse::Ok().json(serde_json::json!({ "short_url": full_short_url }))
}
pub async fn get_url(pool: web::Data<RedisPool>, short_code: web::Path<String>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    let serialized_url: Option<String> = conn.get(&*short_code).unwrap();

    if let Some(serialized_url) = serialized_url {
        let mut shortened_url: models::ShortenedURL =
            serde_json::from_str(&serialized_url).unwrap();
        {
            shortened_url.access_count += 1;
            shortened_url.updated_at = Utc::now();
        }
        let updated_serialized = serde_json::to_string(&shortened_url).unwrap();
        let _: () = conn.set(&*short_code, updated_serialized).unwrap();

        HttpResponse::Ok().json(shortened_url.url)
    } else {
        HttpResponse::NotFound().body("URL not found")
    }
}

pub async fn delete_url(pool: web::Data<RedisPool>, short_code: web::Path<String>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    let deleted: usize = conn.del(&*short_code).unwrap();

    if deleted > 0 {
        HttpResponse::Ok().body("URL successfully deleted")
    } else {
        HttpResponse::NotFound().body("URL not found")
    }
}

pub async fn get_url_stats(
    pool: web::Data<RedisPool>,
    short_code: web::Path<String>,
) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    let serialized_url: Option<String> = conn.get(&*short_code).unwrap();

    if let Some(serialized_url) = serialized_url {
        let shortened_url: models::ShortenedURL =
            serde_json::from_str(&serialized_url).unwrap();
        HttpResponse::Ok().json(shortened_url)
    } else {
        HttpResponse::NotFound().body("URL now found")
    }
}
