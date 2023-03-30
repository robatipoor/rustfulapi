use fake::{Fake, Faker};
use model::request::*;
use test_context::test_context;

use crate::{context::app::AppTestContext, helper::result::AppResponseResult};

#[test_context(AppTestContext)]
#[tokio::test]
pub async fn test_active_user(ctx: &mut AppTestContext) {
  let req: RegisterRequest = Faker.fake();
  let (status, body) = ctx.api.register(&req).await.unwrap();
  assert!(status.is_success());
  let code = ctx.mail.get_code_from_email(&req.email).await.unwrap();
  let resp = body.unwrap();
  let active_req = ActiveRequest {
    id: resp.id,
    code: code.clone(),
  };
  let (status, body) = ctx.api.active(&active_req).await.unwrap();
  assert!(matches!(body, AppResponseResult::Ok(_)));
  assert!(status.is_success());
  let (status, body) = ctx.api.active(&active_req).await.unwrap();
  assert!(!status.is_success());
  assert!(body.is_err());
}
