use fake::{Fake, Faker};
use rustfulapi::dto::{request::*, LoginResponse};
use test_context::test_context;

use crate::{assert_ok, context::app::AppTestContext};

#[test_context(AppTestContext)]
#[tokio::test]
pub async fn test_active_user(ctx: &mut AppTestContext) {
  let req: RegisterRequest = Faker.fake();
  let (status, resp) = ctx.api.register(&req).await.unwrap();
  assert!(status.is_success(), "status: {status}");
  let (code, user_id) = ctx
    .mail
    .get_code_and_id_from_email(&req.email)
    .await
    .unwrap();
  assert_ok!(resp);
  let active_req = ActiveRequest {
    user_id,
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
  assert_ok!(resp, |d| matches!(d, &LoginResponse::Token(_)));
  assert!(status.is_success(), "status: {status}");
}
