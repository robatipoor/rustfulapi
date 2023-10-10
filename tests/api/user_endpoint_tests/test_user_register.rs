use crate::assert_err;
use crate::context::app::AppTestContext;
use crate::helper::result::AppResponseResult;
use error::AppResponseError;
use fake::{Fake, Faker};
use model::request::*;
use model::response::*;
use test_context::test_context;

#[test_context(AppTestContext)]
#[tokio::test]
pub async fn test_success_register_user(ctx: &mut AppTestContext) {
  let req: RegisterRequest = Faker.fake();
  let (status, body) = ctx.api.register(&req).await.unwrap();
  assert!(matches!(
    body,
    AppResponseResult::Ok(RegisterResponse { .. })
  ));
  assert!(status.is_success(), "status: {status}");
  let (status, body) = ctx.api.register(&req).await.unwrap();
  assert_err!(body, |e: &AppResponseError| e.error == "ALREADY_EXISTS");
  assert!(!status.is_success(), "status: {status}");
}
