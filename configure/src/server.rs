use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
  pub addr: String,
  pub port: u16,
}

impl ServerConfig {
  pub fn get_addr(&self) -> String {
    format!("{}:{}", self.addr, self.port)
  }

  pub fn get_http_addr(&self) -> String {
    format!("http://{}:{}", self.addr, self.port)
  }
}

#[cfg(test)]
pub mod tests {

  use super::*;

  #[test]
  pub fn app_config_http_addr_test() {
    let config = ServerConfig {
      addr: "127.0.0.1".to_string(),
      port: 1024,
    };
    assert_eq!(config.get_http_addr(), "http://127.0.0.1:1024");
  }
}
