use test_context::test_context;

use crate::context::app::AppTestContext;

#[test_context(AppTestContext)]
#[tokio::test]
pub async fn test_server_state(ctx: &mut AppTestContext) {
  let (status, body) = ctx.api.server_state().await.unwrap();
  assert!(status.is_success());
  let body = body.unwrap();
  assert!(body.postgres);
  assert!(body.email);
  assert!(body.redis);
}
