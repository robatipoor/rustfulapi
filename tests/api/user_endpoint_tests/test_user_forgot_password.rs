use entity::role::RoleUser;
use test_context::test_context;

use crate::context::seeder::SeedDbTestContext;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_forget_password(ctx: &mut SeedDbTestContext) {
  let user = ctx.users.get(&RoleUser::User).unwrap();
  let (status, _body) = ctx.app.api.forget_password(&user.email).await.unwrap();
  assert!(status.is_success(), "status: {status}");
}
