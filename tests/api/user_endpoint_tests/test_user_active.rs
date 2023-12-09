use fake::{Fake, Faker};
use rustfulapi::dto::request::*;
use test_context::test_context;

use crate::{assert_ok, context::app::AppTestContext, unwrap};

#[test_context(AppTestContext)]
#[tokio::test]
pub async fn test_active_user(ctx: &mut AppTestContext) {
  let req: RegisterRequest = Faker.fake();
  let (status, resp) = ctx.api.register(&req).await.unwrap();
  assert!(status.is_success(), "status: {status}");
  let code = ctx.mail.get_code_from_email(&req.email).await.unwrap();
  let resp = unwrap!(resp);
  let active_req = ActiveRequest {
    user_id: resp.id,
    code: code.clone(),
  };
  let (status, resp) = ctx.api.active(&active_req).await.unwrap();
  assert_ok!(resp);
  assert!(status.is_success(), "status: {status}");
  let (status, resp) = ctx.api.active(&active_req).await.unwrap();
  assert_ok!(resp);
  assert!(status.is_success(), "status: {status}");
  let req = LoginRequest {
    email: req.email.clone(),
    password: req.password.clone(),
  };
  let (status, resp) = ctx.api.login(&req).await.unwrap();
  let resp = unwrap!(resp);
  assert!(status.is_success(), "status: {status}");
}
