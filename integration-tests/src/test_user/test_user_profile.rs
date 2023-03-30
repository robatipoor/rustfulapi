use crate::context::seeder::SeedDbTestContext;
use entity::role::RoleUser;
use fake::{Fake, Faker};
use model::request::*;
use model::response::*;
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_get_profile_user(ctx: &mut SeedDbTestContext) {
  let user = ctx.users.get(&RoleUser::User).unwrap();
  let req = LoginRequest::Normal(NormalLogin {
    email: user.email.clone(),
    password: user.password.clone(),
  });
  let (status, body) = ctx.app.api.login(&req).await.unwrap();
  assert!(status.is_success());
  match body.unwrap() {
    LoginResponse::Token { access_token, .. } => {
      let (status, body) = ctx.app.api.get_profile(&access_token).await.unwrap();
      assert!(status.is_success());
      assert!(!body.unwrap().username.is_empty());
    }
    LoginResponse::Id { id } => {
      panic!("get_profile_user_test failed: {id}");
    }
  }
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_update_profile_user(ctx: &mut SeedDbTestContext) {
  let user = ctx.users.get(&RoleUser::User).unwrap();
  let req = LoginRequest::Normal(NormalLogin {
    email: user.email.clone(),
    password: user.password.clone(),
  });
  let (status, body) = ctx.app.api.login(&req).await.unwrap();
  assert!(status.is_success());
  match body.unwrap() {
    LoginResponse::Token { access_token, .. } => {
      let (status, body) = ctx.app.api.get_profile(&access_token).await.unwrap();
      assert!(status.is_success());
      assert!(!body.unwrap().username.is_empty());
      let req = UpdateProfileRequest {
        username: Some(Faker.fake()),
        ..Default::default()
      };
      let (status, body) = ctx
        .app
        .api
        .update_profile(&access_token, &req)
        .await
        .unwrap();
      assert!(status.is_success());
      assert!(body.is_ok());
      let (status, body) = ctx.app.api.get_profile(&access_token).await.unwrap();
      assert!(status.is_success());
      assert_eq!(body.unwrap().username, req.username.unwrap());
    }
    LoginResponse::Id { id } => {
      panic!("update_profile_user_test failed: {id}");
    }
  }
}
