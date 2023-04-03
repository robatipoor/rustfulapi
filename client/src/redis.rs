use async_trait::async_trait;
use configure::{redis::RedisConfig, CONFIG};
use redis::{Client, RedisError};
use std::time::Duration;
use test_context::AsyncTestContext;
use tracing::log::info;

pub static REDIS: once_cell::sync::Lazy<RedisClient> =
  once_cell::sync::Lazy::new(|| RedisClient::new(&CONFIG.redis).unwrap());

pub type RedisClient = redis::Client;

#[async_trait]
pub trait RedisClientExt: Sized {
  fn new(config: &RedisConfig) -> Result<Self, RedisError>;
  async fn ping(&self) -> Result<Option<String>, RedisError>;
  async fn set(&self, key: &str, value: &str, expire: Duration) -> Result<(), RedisError>;
  async fn exist(&self, key: &str) -> Result<bool, RedisError>;
  async fn get(&self, key: &str) -> Result<Option<String>, RedisError>;
  async fn del(&self, key: &str) -> Result<bool, RedisError>;
  async fn ttl(&self, key: &str) -> Result<i64, RedisError>;
}

pub struct RedisTestContext {
  pub config: RedisConfig,
  pub redis: RedisClient,
}

#[async_trait::async_trait]
impl AsyncTestContext for RedisTestContext {
  async fn setup() -> Self {
    let config = CONFIG.redis.clone();
    info!("setup redis config for the test");
    // let database_name = util::string::generate_random_string_with_prefix("test_db");
    let redis = RedisClient::new(&config).unwrap();
    Self { config, redis }
  }

  async fn teardown(self) {
    // TODO drop db
  }
}

#[async_trait]
impl RedisClientExt for Client {
  fn new(config: &RedisConfig) -> Result<Self, RedisError> {
    redis::Client::open(config.get_url())
  }

  async fn ping(&self) -> Result<Option<String>, RedisError> {
    let mut conn = self.get_async_connection().await?;
    let value: Option<String> = redis::cmd("PING").query_async(&mut conn).await?;
    info!("ping redis server");
    Ok(value)
  }

  async fn set(&self, key: &str, value: &str, expire: Duration) -> Result<(), RedisError> {
    let mut conn = self.get_async_connection().await?;
    let msg: String = redis::cmd("SET")
      .arg(&[key, value])
      .query_async(&mut conn)
      .await?;
    info!("set key redis: {msg}");
    let msg: i32 = redis::cmd("EXPIRE")
      .arg(&[key, &*expire.as_secs().to_string()])
      .query_async(&mut conn)
      .await?;
    info!("set expire time redis: {msg}");
    Ok(())
  }

  async fn exist(&self, key: &str) -> Result<bool, RedisError> {
    let mut conn = self.get_async_connection().await?;
    let value: bool = redis::cmd("EXISTS").arg(key).query_async(&mut conn).await?;
    info!("check key exists: {key}");
    Ok(value)
  }

  async fn get(&self, key: &str) -> Result<Option<String>, RedisError> {
    let mut conn = self.get_async_connection().await?;
    let value: Option<String> = redis::cmd("GET").arg(key).query_async(&mut conn).await?;
    info!("get value: {key}");
    Ok(value)
  }

  async fn del(&self, key: &str) -> Result<bool, RedisError> {
    let mut conn = self.get_async_connection().await?;
    let value: i32 = redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
    info!("delete value: {key}");
    Ok(value == 1)
  }
  async fn ttl(&self, key: &str) -> Result<i64, RedisError> {
    let mut conn = self.get_async_connection().await?;
    let value: i64 = redis::cmd("TTL").arg(key).query_async(&mut conn).await?;
    info!("get TTL value: {key}");
    Ok(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use fake::{Fake, Faker};
  use uuid::Uuid;

  #[tokio::test]
  async fn test_ping_redis_server() {
    let resp = REDIS.ping().await.unwrap();
    let pong = "PONG";
    assert!(matches!(resp, Some(p) if p == pong));
  }

  #[tokio::test]
  async fn test_set_key_redis() {
    let key: String = Faker.fake();
    let value = Uuid::new_v4().to_string();
    REDIS
      .set(&key, &value, Duration::from_secs(5))
      .await
      .unwrap();
    let resp = REDIS.get(&key).await.unwrap();
    assert!(matches!(resp, Some(v) if v == value));
    let resp = REDIS.ttl(&key).await.unwrap();
    assert!(resp > 0);
  }

  #[tokio::test]
  async fn test_exist_key_redis() {
    let key: String = Faker.fake();
    let value = Uuid::new_v4().to_string();
    REDIS
      .set(&key, &value, Duration::from_secs(4))
      .await
      .unwrap();
    let resp = REDIS.get(&key).await.unwrap();
    assert!(matches!(resp, Some(v) if v == value));
    let resp = REDIS.exist(&key).await.unwrap();
    assert!(resp);
    let key: String = Faker.fake();
    let resp = REDIS.exist(&key).await.unwrap();
    assert!(!resp);
  }

  #[tokio::test]
  async fn test_del_key_redis() {
    let key: String = Faker.fake();
    let value = Uuid::new_v4().to_string();
    REDIS
      .set(&key, &value, Duration::from_secs(4))
      .await
      .unwrap();
    let resp = REDIS.get(&key).await.unwrap();
    assert!(matches!(resp, Some(v) if v == value));
    let resp = REDIS.exist(&key).await.unwrap();
    assert!(resp);
    REDIS.del(&key).await.unwrap();
    let resp = REDIS.exist(&key).await.unwrap();
    assert!(!resp);
  }

  #[tokio::test]
  async fn test_key_ttl_redis() {
    let key: String = Faker.fake();
    let ttl = 4;
    let value = Uuid::new_v4().to_string();
    REDIS
      .set(&key, &value, Duration::from_secs(ttl))
      .await
      .unwrap();
    let resp = REDIS.get(&key).await.unwrap();
    assert!(matches!(resp, Some(v) if v == value));
    let resp = REDIS.ttl(&key).await.unwrap();
    assert!(resp <= ttl as i64 && resp > 0);
    REDIS.del(&key).await.unwrap();
    let resp = REDIS.ttl(&key).await.unwrap();
    assert!(resp < 0);
  }
}
