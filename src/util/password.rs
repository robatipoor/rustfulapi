use super::hash;
use crate::error::{invalid_input_error, AppResult};
use tracing::debug;

pub async fn hash(password: String) -> AppResult<String> {
  let jh = tokio::task::spawn_blocking(move || hash::argon_hash(password));
  let password = jh.await??;
  Ok(password)
}

pub async fn verify(password: String, hashed_pass: String) -> AppResult {
  let jh = tokio::task::spawn_blocking(move || hash::argon_verify(password, hashed_pass));
  if let Err(e) = jh.await? {
    debug!("The password is not correct: {e}");
    Err(invalid_input_error(
      "password",
      "The password is not correct.",
    ))
  } else {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use fake::{Fake, Faker};

  use super::*;

  #[tokio::test]
  pub async fn test_password_hash() {
    let password: String = Faker.fake();
    let hash_pass = hash(password).await.unwrap();
    assert!(!hash_pass.is_empty());
  }

  #[tokio::test]
  pub async fn test_password_hash_and_then_verify_it() {
    let password: String = Faker.fake();
    let hash_pass = hash(password.clone()).await.unwrap();
    verify(password, hash_pass).await.unwrap();
  }
}
