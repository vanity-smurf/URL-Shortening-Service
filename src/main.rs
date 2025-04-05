use actix_web::{App, HttpServer, web};

mod handlers;
mod models;
mod redis;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_url = "redis://127.0.0.1/";
    let redis_pool = redis::init_redis_pool(&redis_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_pool.clone()))
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
