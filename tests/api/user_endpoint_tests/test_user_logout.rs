use crate::{assert_ok, context::seeder::SeedDbTestContext};
use rustfulapi::{dto::LoginRequest, entity::role::RoleUser};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_logout(ctx: &mut SeedDbTestContext) {
  let user = ctx.users.get(&RoleUser::User).unwrap();
  let req = LoginRequest {
    email: user.email.clone(),
    password: user.password.clone(),
  };
  let token = ctx.app.api.get_token(&req).await.unwrap();
  let (status, resp) = ctx.app.api.logout(&token.access_token).await.unwrap();
  assert_ok!(resp);
  assert!(status.is_success(), "status: {status}");
}
