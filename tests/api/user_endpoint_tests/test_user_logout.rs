use crate::{assert_err, assert_ok, context::seeder::SeedDbTestContext, unwrap};
use reqwest::StatusCode;
use rustfulapi::{
  dto::{LoginRequest, LoginResponse},
  entity::role::RoleUser,
  error::AppResponseError,
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_logout(ctx: &mut SeedDbTestContext) {
  let user = ctx.users.get(&RoleUser::User).unwrap();
  let req = LoginRequest {
    email: user.email.clone(),
    password: user.password.clone(),
  };
  let (status, resp) = ctx.app.api.login(&req).await.unwrap();
  let resp = unwrap!(resp);
  assert!(status.is_success(), "status: {status}");
  match resp {
    LoginResponse::Token(token) => {
      let (status, resp) = ctx.app.api.logout(&token.access_token).await.unwrap();
      assert_ok!(resp);
      assert!(status.is_success(), "status: {status}");
    }
    LoginResponse::Code { .. } => {
      panic!("It was not expected to receive message.");
    }
  }
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_invalid_token(ctx: &mut SeedDbTestContext) {
  use fake::Fake;
  let token: String = fake::Faker.fake();
  let (status, resp) = ctx.app.api.logout(&token).await.unwrap();
  assert_err!(resp, |e: &AppResponseError| e.kind == "UNAUTHORIZED_ERROR");
  assert!(status == StatusCode::UNAUTHORIZED, "status: {status}");
}
