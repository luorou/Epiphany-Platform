use initialize::{database::init_database, redis::init_redis};
use state::Container;

pub mod domain;
pub mod handler;
pub mod initialize;
pub mod middleware;
pub mod service;

pub static APP_CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();

pub async fn init_context() {
    init_database().await;
    init_redis().await;
}
