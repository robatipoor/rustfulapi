use test_context::test_context;

use crate::context::app::AppTestContext;

#[test_context(AppTestContext)]
#[tokio::test]
pub async fn test_server_health_check(ctx: &mut AppTestContext) {
  let (status, body) = ctx.api.health_check().await.unwrap();
  assert!(body.is_ok());
  assert!(status.is_success());
}
