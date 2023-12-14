use rustfulapi::dto::MessageResponse;
use test_context::test_context;

use crate::{assert_ok, context::app::AppTestContext};

#[test_context(AppTestContext)]
#[tokio::test]
pub async fn test_server_health_check(ctx: &mut AppTestContext) {
  let (status, resp) = ctx.api.health_check().await.unwrap();
  assert_ok!(resp, |r: &MessageResponse| r.message == "Ok");
  assert!(status.is_success());
}
