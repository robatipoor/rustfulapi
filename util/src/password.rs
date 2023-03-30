use super::hash;
use error::{invalid_input_error, AppResult};
use tracing::debug;

pub async fn hash(password: String) -> AppResult<String> {
  let jh = tokio::task::spawn_blocking(move || hash::argon_hash(password));
  let password = jh.await??;
  Ok(password)
}

pub async fn verify(password: String, hashed_pass: String) -> AppResult {
  let jh = tokio::task::spawn_blocking(move || hash::argon_verify(password, hashed_pass));
  if let Err(e) = jh.await? {
    debug!("password is incorrect error: {e}");
    Err(invalid_input_error("password", "Password is Incorrect"))
  } else {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use fake::{Fake, Faker};

  use super::*;

  #[tokio::test]
  pub async fn hash_service_test() {
    let password: String = Faker.fake();
    let hash_pass = hash(password).await.unwrap();
    assert!(!hash_pass.is_empty());
  }

  #[tokio::test]
  pub async fn verify_hash_service_test() {
    let password: String = Faker.fake();
    let hash_pass = hash(password.clone()).await.unwrap();
    let result = verify(password, hash_pass).await;
    assert!(result.is_ok());
  }
}
