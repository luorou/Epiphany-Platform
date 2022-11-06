use initialize::{database::init_database, redis_cache::init_redis};
use state::Container;
pub mod initialize;
pub mod handler;
pub mod middleware;
pub mod caches;
pub mod domain;
pub mod service;

pub static APP_CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();

pub async fn init_context() {
    init_database().await;
    init_redis().await;
}