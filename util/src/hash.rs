use argon2::{
  password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
  Argon2,
};

pub fn argon_hash(content: impl AsRef<str>) -> Result<String, argon2::password_hash::Error> {
  let salt = SaltString::generate(&mut OsRng);
  let argon = Argon2::default();
  Ok(
    argon
      .hash_password(content.as_ref().as_bytes(), &salt)?
      .to_string(),
  )
}

pub fn argon_verify(
  content: impl AsRef<str>,
  hash: impl AsRef<str>,
) -> Result<(), argon2::password_hash::Error> {
  let parsed_hash = PasswordHash::new(hash.as_ref())?;
  Argon2::default().verify_password(content.as_ref().as_bytes(), &parsed_hash)
}

#[cfg(test)]
mod tests {
  use fake::{Fake, Faker};

  use super::*;

  #[test]
  pub fn test_argon_hash() {
    let password: String = Faker.fake();
    let hash_pass = argon_hash(password).unwrap();
    assert!(!hash_pass.is_empty());
  }

  #[test]
  pub fn test_verify_argon() {
    let password: String = Faker.fake();
    let hash_pass = argon_hash(password.clone()).unwrap();
    let result = argon_verify(password, hash_pass);
    assert!(result.is_ok());
  }
}
