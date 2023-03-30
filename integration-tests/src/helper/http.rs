use std::time::Duration;

use once_cell::sync::Lazy;
use wiremock::{matchers::*, Request};
use wiremock::{Mock, MockServer, ResponseTemplate};

pub async fn http_mock_server() -> MockServer {
  let mock_server = MockServer::start().await;
  let mock = Mock::given(any()).respond_with(|_req: &Request| ResponseTemplate::new(200));
  mock_server.register(mock).await;
  mock_server
}

pub static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
  reqwest::Client::builder()
    .timeout(Duration::from_secs(120))
    .build()
    .unwrap()
});
