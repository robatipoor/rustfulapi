use std::fmt::Debug;
use std::fmt::Display;
use std::time::Duration;

use client::redis::RedisClientExt;
use constant::*;
use fake::Dummy;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use client::redis::RedisClient;
use error::AppResult;

pub trait RedisKey: Debug + Display {
  type Value: Serialize + DeserializeOwned + Debug;
  const EXPIRE_TIME: Duration;
  fn expire(&self) -> Duration {
    Self::EXPIRE_TIME
  }
}

#[derive(Debug, Serialize, Deserialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct BlockEmailKey {
  pub email: String,
}

impl RedisKey for BlockEmailKey {
  type Value = BlockValue;
  const EXPIRE_TIME: Duration = EXPIRE_BLOCKED_EMAIL_SECS;
}

impl Display for BlockEmailKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "BLOCK_EMAIL_KEY{}", self.email)
  }
}

#[derive(Debug, Serialize, Deserialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct SessionKey {
  pub user_id: Uuid,
}

impl RedisKey for SessionKey {
  type Value = SessionValue;
  const EXPIRE_TIME: Duration = EXPIRE_SESSION_CODE_SECS;
}

impl Display for SessionKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "SESSION_KEY_{}", self.user_id)
  }
}

#[derive(Debug, Serialize, Deserialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct InvitationKey {
  pub id: Uuid,
}

impl RedisKey for InvitationKey {
  type Value = UserValue;
  const EXPIRE_TIME: Duration = EXPIRE_INVITATION_CODE_SECS;
}

impl Display for InvitationKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "INVITATION_KEY_{}", self.id)
  }
}

#[derive(Debug, Serialize, Deserialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct ForgetPasswordKey {
  pub id: Uuid,
}

impl RedisKey for ForgetPasswordKey {
  type Value = UserValue;
  const EXPIRE_TIME: Duration = EXPIRE_FORGET_PASS_CODE_SECS;
}

impl Display for ForgetPasswordKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "FORGET_PASS_KEY_{}", self.id)
  }
}

#[derive(Debug, Serialize, Deserialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct TwoFactorLoginKey {
  pub id: Uuid,
}

impl RedisKey for TwoFactorLoginKey {
  type Value = UserValue;
  const EXPIRE_TIME: Duration = EXPIRE_TWO_FACTOR_CODE_SECS;
}

impl Display for TwoFactorLoginKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "TWO_FACTOR_LOGIN_KEY_{}", self.id)
  }
}

#[derive(Debug, Serialize, Deserialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct BlockValue {
  pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct UserValue {
  pub user_id: Uuid,
  pub code: String,
}

#[derive(Debug, Serialize, Deserialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct SessionValue {
  pub user_id: Uuid,
  pub id: Uuid,
}

impl UserValue {
  pub fn new<T: Into<String>>(user_id: Uuid, code: T) -> Self {
    Self {
      user_id,
      code: code.into(),
    }
  }
}

pub async fn set<K>(client: &RedisClient, (key, value): (&K, &K::Value)) -> AppResult<()>
where
  K: RedisKey,
{
  info!("set value to redis key : {:?} value : {:?}", key, value);
  let value = serde_json::to_string(value)?;
  client.set(&key.to_string(), &value, K::EXPIRE_TIME).await?;
  Ok(())
}

pub async fn get<K>(client: &RedisClient, key: &K) -> AppResult<Option<K::Value>>
where
  K: RedisKey,
{
  info!("get value from redis key :{key} ");
  Ok(
    client
      .get(&key.to_string())
      .await?
      .map(|v| serde_json::from_str::<K::Value>(&v))
      .transpose()?,
  )
}
pub async fn del(client: &RedisClient, key: &impl RedisKey) -> Result<bool, redis::RedisError> {
  info!("delete key in redis : {:?}", key);
  client.del(&key.to_string()).await
}

pub async fn pull<K>(client: &RedisClient, key: &K) -> AppResult<Option<K::Value>>
where
  K: RedisKey,
{
  info!("get and delete key from redis key : {:?}", key);
  let result = get(client, key).await?;
  if result.is_some() {
    del(client, key).await?;
  }
  Ok(result)
}

pub async fn check_exist_key(redis: &RedisClient, key: &impl RedisKey) -> AppResult<bool> {
  Ok(redis.exist(&key.to_string()).await?)
}

#[cfg(test)]
mod tests {
  use client::redis::REDIS;
  use fake::{Fake, Faker};

  use super::*;

  #[tokio::test]
  async fn test_set_and_get_str_redis_service() {
    let key: SessionKey = Faker.fake();
    let value: SessionValue = Faker.fake();
    set(&REDIS, (&key, &value)).await.unwrap();
    let actual_value = get(&REDIS, &key).await.unwrap().unwrap();
    assert_eq!(actual_value, value);
  }

  #[tokio::test]
  async fn test_pull_redis_service() {
    let key: SessionKey = Faker.fake();
    let value: SessionValue = Faker.fake();
    set(&REDIS, (&key, &value)).await.unwrap();
    let actual_value = pull(&REDIS, &key).await.unwrap().unwrap();
    assert_eq!(actual_value, value);
    let actual_value = get(&REDIS, &key).await.unwrap();
    assert!(actual_value.is_none());
  }

  #[tokio::test]
  async fn test_delete_redis_service() {
    let key: TwoFactorLoginKey = Faker.fake();
    let value: UserValue = Faker.fake();
    set(&REDIS, (&key, &value)).await.unwrap();
    let actual_value = get(&REDIS, &key).await.unwrap().unwrap();
    assert_eq!(actual_value, value);
    let actual_value = del(&REDIS, &key).await.unwrap();
    assert!(actual_value);
    let actual_value = get(&REDIS, &key).await.unwrap();
    assert!(actual_value.is_none());
  }

  #[tokio::test]
  async fn test_set_and_get_value_redis_service() {
    let key: InvitationKey = Faker.fake();
    let value: UserValue = Faker.fake();
    set(&REDIS, (&key, &value)).await.unwrap();
    let actual_value = get(&REDIS, &key).await.unwrap().unwrap();
    assert_eq!(actual_value, value);
  }
}
