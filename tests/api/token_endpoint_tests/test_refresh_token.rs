use crate::unwrap;
use crate::{assert_err, context::seeder::SeedDbTestContext};
use rustfulapi::{
  dto::{LoginRequest, LoginResponse, RefreshTokenRequest},
  error::AppResponseError,
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_refresh_token(ctx: &mut SeedDbTestContext) {
  let user = ctx
    .users
    .get(&rustfulapi::entity::role::RoleUser::User)
    .unwrap();
  let req = LoginRequest {
    email: user.email.clone(),
    password: user.password.clone(),
  };
  let (status, resp) = ctx.app.api.login(&req).await.unwrap();
  let resp = unwrap!(resp);
  assert!(status.is_success(), "status: {status:?}");
  match resp {
    LoginResponse::Token(token) => {
      let (status, _) = ctx.app.api.get_profile(&token.access_token).await.unwrap();
      assert!(status.is_success(), "status: {status:?}");
      let req = RefreshTokenRequest {
        token: token.refresh_token,
      };
      let (status, resp) = ctx.app.api.refresh_token(&req).await.unwrap();
      assert!(status.is_success(), "status: {status:?}");
      let second_token = unwrap!(resp);
      assert!(!second_token.access_token.is_empty());
      assert!(!second_token.refresh_token.is_empty());
      let (status, resp) = ctx.app.api.refresh_token(&req).await.unwrap();
      assert_err!(resp, |e: &AppResponseError| e.kind
        == "INVALID_SESSION_ERROR");
      assert!(!status.is_success(), "status: {status:?}");
      let (status, _) = ctx
        .app
        .api
        .get_profile(&second_token.access_token)
        .await
        .unwrap();
      assert!(!status.is_success(), "status: {status:?}");
    }
    LoginResponse::Code { .. } => {
      panic!("refresh_token_test failed.");
    }
  }
}
