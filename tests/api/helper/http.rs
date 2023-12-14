use wiremock::{matchers::*, Request};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[allow(dead_code)]
pub async fn http_mock_server() -> MockServer {
  let mock_server = MockServer::start().await;
  let mock = Mock::given(any()).respond_with(|_req: &Request| ResponseTemplate::new(200));
  mock_server.register(mock).await;
  mock_server
}
