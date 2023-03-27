use crate::web_server::redis_store_with_pool::RedisStore;
use std::env;
use tower_sessions::fred::prelude::*;

pub async fn create_session_store() -> RedisStore {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL env var not found");
    let redis_config = RedisConfig::from_url(redis_url.as_str()).unwrap();
    let redis_pool = Builder::from_config(redis_config)
        .set_policy(ReconnectPolicy::new_exponential(3, 100, 30_000, 2))
        .build_pool(5)
        .unwrap();
    redis_pool.connect();
    redis_pool
        .wait_for_connect()
        .await
        .expect("Couldn't connect to redis sessions store");
    RedisStore::new(redis_pool)
}
