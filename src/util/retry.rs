use tracing::warn;

use crate::constant::{MAX_RETRY, MINIMUM_DELAY_TIME};

#[async_trait::async_trait]
pub trait RetryUntil<F, T, P>: Send + Sync
where
  P: Fn(&T) -> bool,
{
  async fn retry_until(self, predicate: P) -> T;
}

#[async_trait::async_trait]
impl<F, T, P, Fut> RetryUntil<F, T, P> for F
where
  Fut: std::future::Future<Output = T> + Send + Sync,
  F: Fn() -> Fut + Send + Sync,
  T: Send + Sync,
  P: Fn(&T) -> bool + Send + Sync + 'static,
{
  async fn retry_until(self, predicate: P) -> T {
    let mut remaining_attempts = MAX_RETRY;
    let mut delay = MINIMUM_DELAY_TIME;
    loop {
      remaining_attempts -= 1;
      let result = self().await;
      if predicate(&result) {
        return result;
      } else if remaining_attempts == 0 {
        warn!("Maximum number of attempts exceeded");
        return result;
      }
      tokio::time::sleep(delay).await;
      delay += MINIMUM_DELAY_TIME;
    }
  }
}

#[async_trait::async_trait]
pub trait RetryResult<F, T, E>: Send + Sync {
  async fn until_ok(self) -> Result<T, E>;
}

#[async_trait::async_trait]
impl<F, T, E, Fut> RetryResult<F, T, E> for F
where
  Fut: std::future::Future<Output = Result<T, E>> + Send + Sync,
  F: Fn() -> Fut + Send + Sync,
  E: Send + Sync,
  T: Send + Sync,
{
  async fn until_ok(self) -> Result<T, E> {
    let mut remaining_attempts = MAX_RETRY;
    let mut delay = MINIMUM_DELAY_TIME;
    loop {
      remaining_attempts -= 1;
      let result = self().await;
      if result.is_ok() {
        return result;
      } else if remaining_attempts == 0 {
        warn!("Maximum number of attempts exceeded");
        return result;
      }
      tokio::time::sleep(delay).await;
      delay += MINIMUM_DELAY_TIME;
    }
  }
}
