use fake::{Fake, Faker};
use model::request::*;
use test_context::test_context;

use crate::{assert_err, assert_ok, context::app::AppTestContext, unwrap};

#[test_context(AppTestContext)]
#[tokio::test]
pub async fn test_active_user(ctx: &mut AppTestContext) {
  let req: RegisterRequest = Faker.fake();
  let (status, body) = ctx.api.register(&req).await.unwrap();
  assert!(status.is_success(), "status: {status}");
  let code = ctx.mail.get_code_from_email(&req.email).await.unwrap();
  let body = unwrap!(body);
  let active_req = ActiveRequest {
    id: body.id,
    code: code.clone(),
  };
  let (status, body) = ctx.api.active(&active_req).await.unwrap();
  assert_ok!(body);
  assert!(status.is_success(), "status: {status}");
  let (status, body) = ctx.api.active(&active_req).await.unwrap();
  assert_err!(body);
  assert!(!status.is_success(), "status: {status}");
}
