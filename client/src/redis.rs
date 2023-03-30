use async_trait::async_trait;
use configure::redis::RedisConfig;
use redis::{Client, RedisError};
use std::time::Duration;
use tracing::log::info;

pub type RedisClient = redis::Client;

#[async_trait]
pub trait RedisClientExt: Sized {
  async fn new(config: &RedisConfig) -> Result<Self, RedisError>;
  async fn ping(&self) -> Result<Option<String>, RedisError>;
  async fn set(&self, key: &str, value: &str, expire: Duration) -> Result<(), RedisError>;
  async fn exist(&self, key: &str) -> Result<bool, RedisError>;
  async fn get(&self, key: &str) -> Result<Option<String>, RedisError>;
  async fn del(&self, key: &str) -> Result<bool, RedisError>;
  async fn ttl(&self, key: &str) -> Result<i32, RedisError>;
}

#[async_trait]
impl RedisClientExt for Client {
  async fn new(config: &RedisConfig) -> Result<Self, RedisError> {
    redis::Client::open(config.get_url())
  }

  async fn ping(&self) -> Result<Option<String>, RedisError> {
    let mut conn = self.get_async_connection().await?;
    let value: Option<String> = redis::cmd("PING").query_async(&mut conn).await?;
    info!("ping redis");
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
    info!("check key exists in redis: {key}");
    Ok(value)
  }

  async fn get(&self, key: &str) -> Result<Option<String>, RedisError> {
    let mut conn = self.get_async_connection().await?;
    let value: Option<String> = redis::cmd("GET").arg(key).query_async(&mut conn).await?;
    info!("get value redis: {key}");
    Ok(value)
  }

  async fn del(&self, key: &str) -> Result<bool, RedisError> {
    let mut conn = self.get_async_connection().await?;
    let value: i32 = redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
    info!("del value redis: {key}");
    Ok(value == 1)
  }
  async fn ttl(&self, key: &str) -> Result<i32, RedisError> {
    let mut conn = self.get_async_connection().await?;
    let value: i32 = redis::cmd("TTL").arg(key).query_async(&mut conn).await?;
    info!("TTL value redis: {key}");
    Ok(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use configure::CONFIG;

  #[tokio::test]
  async fn test_redis_ping() {
    let client = RedisClient::new(&CONFIG.redis).await.unwrap();
    assert_eq!(client.ping().await.unwrap().unwrap(), "PONG");
  }

  // #[test_context(RedisTestContext)]
  // #[tokio::test]
  // async fn set_key_redis_test(ctx: &mut RedisTestContext) {
  //   let key: String = Faker.fake();
  //   let uuid = util::string::generate_random_name(None);
  //   set(&ctx.redis, &key, &uuid, Duration::from_secs(50))
  //     .await
  //     .unwrap();
  //   let actual_uuid = get(&ctx.redis, &key).await.unwrap();
  //   assert_eq!(actual_uuid.unwrap(), uuid);
  //   let result = ttl(&ctx.redis, &key).await.unwrap();
  //   assert!(result > 0);
  // }

  // #[test_context(RedisTestContext)]
  // #[tokio::test]
  // async fn exist_key_redis_test(ctx: &mut RedisTestContext) {
  //   let key: String = Faker.fake();
  //   let value: String = Faker.fake();
  //   set(&ctx.redis, &key, &value, Duration::from_secs(10))
  //     .await
  //     .unwrap();
  //   let actual_value = exist(&ctx.redis, &key).await.unwrap();
  //   assert!(actual_value);
  //   let key: String = Faker.fake();
  //   let actual_value = exist(&ctx.redis, &key).await.unwrap();
  //   assert!(!actual_value);
  // }

  // #[test_context(RedisTestContext)]
  // #[tokio::test]
  // async fn del_key_redis_test(ctx: &mut RedisTestContext) {
  //   let key: String = Faker.fake();
  //   let uuid = util::string::generate_random_name(None);
  //   set(&ctx.redis, &key, &uuid, Duration::from_secs(3))
  //     .await
  //     .unwrap();
  //   let actual_uuid = get(&ctx.redis, &key).await.unwrap();
  //   assert_eq!(actual_uuid.unwrap(), uuid);
  //   let result = del(&ctx.redis, &key).await.unwrap();
  //   assert!(result);
  //   let actual_uuid = get(&ctx.redis, &key).await.unwrap();
  //   assert!(actual_uuid.is_none());
  // }

  // #[test_context(RedisTestContext)]
  // #[tokio::test]
  // async fn ttl_key_redis_test(ctx: &mut RedisTestContext) {
  //   let key: String = Faker.fake();
  //   let uuid = util::string::generate_random_name(None);
  //   set(&ctx.redis, &key, &uuid, Duration::from_secs(60))
  //     .await
  //     .unwrap();
  //   let actual_uuid = get(&ctx.redis, &key).await.unwrap();
  //   assert_eq!(actual_uuid.unwrap(), uuid);
  //   let result = ttl(&ctx.redis, &key).await.unwrap();
  //   assert!(result > 0 && result <= 60);
  // }
}
