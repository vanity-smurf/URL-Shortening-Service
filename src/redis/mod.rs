use r2d2::Pool;
use redis_r2d2::{r2d2, RedisConnectionManager};

pub type RedisPool = Pool<RedisConnectionManager>;

pub fn init_redis_pool(redis_url: &str) -> RedisPool {
    let manager = RedisConnectionManager::new(redis_url)
        .expect("Invalid redis url");

    r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create Radis connection pool")
}
