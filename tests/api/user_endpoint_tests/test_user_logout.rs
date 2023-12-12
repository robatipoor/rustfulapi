use crate::{assert_ok, context::seeder::SeedDbTestContext, unwrap};
use rustfulapi::{
  dto::{LoginRequest, LoginResponse},
  entity::role::RoleUser,
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
