use rand::{distributions::Alphanumeric, Rng};

pub fn generate_random_string(len: usize) -> String {
  rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(len)
    .map(char::from)
    .collect()
}

pub fn generate_random_string_with_prefix(prefix: &str) -> String {
  format!("{prefix}_{}", generate_random_string(10))
}

#[cfg(test)]
mod tests {
  use fake::{Fake, Faker};

  use super::*;

  #[test]
  fn test_generate_random_string_with_prefix() {
    let prefix: String = Faker.fake();
    let result = generate_random_string_with_prefix(&prefix);
    assert!(result.starts_with(&*prefix));
  }

  #[test]
  fn test_generate_random_string() {
    let len = 4;
    let name = generate_random_string(len);
    assert_eq!(name.len(), len);
  }
}
