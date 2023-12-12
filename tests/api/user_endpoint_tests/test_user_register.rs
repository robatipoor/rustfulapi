use crate::assert_err;
use crate::context::app::AppTestContext;
use crate::helper::result::AppResponseResult;
use fake::{Fake, Faker};
use rustfulapi::dto::request::*;
use rustfulapi::dto::response::*;
use rustfulapi::error::AppResponseError;
use test_context::test_context;

#[test_context(AppTestContext)]
#[tokio::test]
pub async fn test_success_register_user(ctx: &mut AppTestContext) {
  let req: RegisterRequest = Faker.fake();
  let (status, resp) = ctx.api.register(&req).await.unwrap();
  assert!(matches!(
    resp,
    AppResponseResult::Ok(RegisterResponse { .. })
  ));
  assert!(status.is_success(), "status: {status}");
  let (status, resp) = ctx.api.register(&req).await.unwrap();
  assert_err!(resp, |e: &AppResponseError| e.kind
    == "USER_ALREADY_EXISTS_ERROR");
  assert!(!status.is_success(), "status: {status}");
}
