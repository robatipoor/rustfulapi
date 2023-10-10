use crate::context::seeder::SeedDbTestContext;
use crate::unwrap;
use entity::role::RoleUser;
use model::request::*;
use model::response::*;
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_refresh_token(ctx: &mut SeedDbTestContext) {
  let user = ctx.users.get(&RoleUser::User).unwrap();
  let req = LoginRequest::Normal(NormalLogin {
    email: user.email.clone(),
    password: user.password.clone(),
  });
  let (status, body) = ctx.app.api.login(&req).await.unwrap();
  let body = unwrap!(body);
  assert!(status.is_success(), "status: {status:?}");
  match body {
    LoginResponse::Token {
      access_token,
      refresh_token,
      ..
    } => {
      let (status, _) = ctx.app.api.get_profile(&access_token).await.unwrap();
      assert!(status.is_success(), "status: {status:?}");
      let (status, body) = ctx.app.api.refresh_token(&refresh_token).await.unwrap();
      assert!(status.is_success(), "status: {status:?}");
      let second_token = unwrap!(body);
      assert!(!second_token.access_token.is_empty());
      assert!(!second_token.refresh_token.is_empty());
      let (status, _body) = ctx.app.api.refresh_token(&refresh_token).await.unwrap();
      assert!(!status.is_success(), "status: {status:?}");
      let (status, _) = ctx
        .app
        .api
        .get_profile(&second_token.access_token)
        .await
        .unwrap();
      assert!(!status.is_success(), "status: {status:?}");
    }
    LoginResponse::Id { id } => {
      panic!("refresh_token_test failed {id:?}");
    }
  }
}
