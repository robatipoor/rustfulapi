use rustfulapi::{
  dto::{LoginRequest, TokenInfoRequest},
  entity::role::RoleUser,
};
use test_context::test_context;

use crate::{assert_ok, context::seeder::SeedDbTestContext};

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_get_token_info(ctx: &mut SeedDbTestContext) {
  let system = ctx.users.get(&RoleUser::System).unwrap();
  let req = LoginRequest {
    email: system.email.clone(),
    password: system.password.clone(),
  };
  let system_token = ctx.app.api.get_token(&req).await.unwrap();
  let user = ctx.users.get(&RoleUser::User).unwrap();
  let req = LoginRequest {
    email: user.email.clone(),
    password: user.password.clone(),
  };
  let user_token = ctx.app.api.get_token(&req).await.unwrap();
  let req = TokenInfoRequest {
    token: user_token.access_token.clone(),
  };
  let (status, resp) = ctx
    .app
    .api
    .token_info(&system_token.access_token, &req)
    .await
    .unwrap();
  assert_ok!(resp);
  assert!(status.is_success(), "status: {status}");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_get_token_with_invalid_access(ctx: &mut SeedDbTestContext) {
  let user = ctx.users.get(&RoleUser::User).unwrap();
  let req = LoginRequest {
    email: user.email.clone(),
    password: user.password.clone(),
  };
  let user_token = ctx.app.api.get_token(&req).await.unwrap();
  let req = TokenInfoRequest {
    token: user_token.access_token.clone(),
  };
  let (status, _resp) = ctx
    .app
    .api
    .token_info(&user_token.access_token, &req)
    .await
    .unwrap();
  assert_eq!(status, reqwest::StatusCode::FORBIDDEN);
}
