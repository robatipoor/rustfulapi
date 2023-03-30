use openssl::error::ErrorStack;
use openssl::rsa::Rsa;

#[derive(Debug)]
pub struct RsaPairKey {
  pub private_key: Vec<u8>,
  pub public_key: Vec<u8>,
}

impl RsaPairKey {
  pub fn new(bits: u32) -> Result<Self, ErrorStack> {
    let rsa = Rsa::generate(bits)?;
    let private_key = rsa.private_key_to_pem()?;
    let public_key = rsa.public_key_to_pem_pkcs1()?;
    Ok(Self {
      private_key,
      public_key,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pair_key_test() {
    assert!(RsaPairKey::new(2048).is_ok());
    assert!(RsaPairKey::new(1024).is_ok());
  }
}
