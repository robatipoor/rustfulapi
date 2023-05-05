use fake::{Fake, Faker};
use model::request::*;
use test_context::test_context;

use crate::{assert_err, assert_ok, context::app::AppTestContext};

#[test_context(AppTestContext)]
#[tokio::test]
pub async fn test_invalid_request_invitation_test(ctx: &mut AppTestContext) {
  let req: RegisterRequest = Faker.fake();
  let (status, body) = ctx.api.register(&req).await.unwrap();
  assert_ok!(body);
  assert!(status.is_success());
  let req = InvitationRequest::new(req.email, req.password);
  let (status, body) = ctx.api.invitation(&req).await.unwrap();
  assert_err!(body);
  assert!(!status.is_success());
}
