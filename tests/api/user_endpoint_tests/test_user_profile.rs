use crate::assert_ok;
use crate::context::seeder::SeedDbTestContext;
use crate::unwrap;
use fake::{Fake, Faker};
use rustfulapi::{
  dto::{LoginRequest, LoginResponse, UpdateProfileRequest},
  entity::role::RoleUser,
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_get_profile_user(ctx: &mut SeedDbTestContext) {
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
      let (status, resp) = ctx.app.api.get_profile(&token.access_token).await.unwrap();
      assert!(status.is_success(), "status: {status}");
      let resp = unwrap!(resp);
      assert!(!resp.username.is_empty());
    }
    LoginResponse::Code { .. } => {
      panic!("get_profile_user_test failed.");
    }
  }
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_update_profile_user(ctx: &mut SeedDbTestContext) {
  let user = ctx.users.get(&RoleUser::User).unwrap();
  let req = LoginRequest {
    email: user.email.clone(),
    password: user.password.clone(),
  };
  let (status, resp) = ctx.app.api.login(&req).await.unwrap();
  let resp = unwrap!(resp);
  assert!(status.is_success());
  assert!(status.is_success(), "status: {status}");
  match resp {
    LoginResponse::Token(token) => {
      let (status, resp) = ctx.app.api.get_profile(&token.access_token).await.unwrap();
      let resp = unwrap!(resp);
      assert!(status.is_success(), "status: {status}");
      assert!(!resp.username.is_empty());
      let req = UpdateProfileRequest {
        username: Some(Faker.fake()),
        ..Default::default()
      };
      let (status, resp) = ctx
        .app
        .api
        .update_profile(&token.access_token, &req)
        .await
        .unwrap();
      assert_ok!(resp);
      assert!(status.is_success(), "status: {status}");
      let (status, resp) = ctx.app.api.get_profile(&token.access_token).await.unwrap();
      let resp = unwrap!(resp);
      assert!(status.is_success(), "status: {status}");
      assert_eq!(resp.username, req.username.unwrap());
    }
    LoginResponse::Code { .. } => {
      panic!("update_profile_user_test failed. ");
    }
  }
}
