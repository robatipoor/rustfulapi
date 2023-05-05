use crate::{assert_ok, context::seeder::SeedDbTestContext, unwrap};
use entity::role::RoleUser;
use model::*;
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_logout_user(ctx: &mut SeedDbTestContext) {
  let user = ctx.users.get(&RoleUser::User).unwrap();
  let req = LoginRequest::Normal(NormalLogin {
    email: user.email.clone(),
    password: user.password.clone(),
  });
  let (status, body) = ctx.app.api.login(&req).await.unwrap();
  let body = unwrap!(body);
  assert!(status.is_success(), "status: {status}");
  match body {
    LoginResponse::Token { access_token, .. } => {
      let (status, body) = ctx.app.api.logout(&access_token).await.unwrap();
      assert_ok!(body);
      assert!(status.is_success(), "status: {status}");
    }
    LoginResponse::Id { id } => {
      panic!("logout user test failed: {id}");
    }
  }
}
