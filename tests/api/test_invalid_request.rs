use crate::{assert_err, context::app::AppTestContext};
use fake::Fake;
use reqwest::StatusCode;
use rustfulapi::error::AppResponseError;
use test_context::test_context;

#[test_context(AppTestContext)]
#[tokio::test]
pub async fn test_invalid_token(ctx: &mut AppTestContext) {
  let token: String = fake::Faker.fake();
  let (status, resp) = ctx.api.logout(&token).await.unwrap();
  assert_err!(resp, |e: &AppResponseError| e.kind == "UNAUTHORIZED_ERROR");
  assert!(status == StatusCode::UNAUTHORIZED, "status: {status}");
}
