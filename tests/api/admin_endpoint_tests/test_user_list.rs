use crate::context::seeder::SeedDbTestContext;
use crate::unwrap;
use rustfulapi::dto::*;
use rustfulapi::entity::role::RoleUser;
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_get_user_list(ctx: &mut SeedDbTestContext) {
  let user = ctx.users.get(&RoleUser::Admin).unwrap();
  let req = LoginRequest {
    email: user.email.clone(),
    password: user.password.clone(),
  };
  let token = ctx.app.api.get_token(&req).await.unwrap();
  let param = PageQueryParam {
    page_num: 0,
    page_size: 5,
    sort_by: None,
    sort_direction: None,
  };
  let (status, resp) = ctx
    .app
    .api
    .get_user_list(&param, &token.access_token)
    .await
    .unwrap();
  let resp = unwrap!(resp);
  assert!(!resp.list.is_empty());
  assert!(status.is_success(), "status: {status}");
}
