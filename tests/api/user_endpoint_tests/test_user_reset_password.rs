use crate::{assert_ok, context::seeder::SeedDbTestContext, unwrap};
use rustfulapi::{
  dto::{LoginRequest, LoginResponse, SetPasswordRequest},
  entity::role::RoleUser,
  util,
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_reset_password(ctx: &mut SeedDbTestContext) {
  let user = ctx.users.get(&RoleUser::User).unwrap();
  let new_password: String = util::random::generate_random_string(10);
  let (status, resp) = ctx.app.api.forget_password(&user.email).await.unwrap();
  assert_ok!(resp);
  assert!(status.is_success(), "status: {status}");
  let (code, user_id) = ctx
    .app
    .mail
    .get_code_and_id_from_email(&user.email)
    .await
    .unwrap();
  let req = SetPasswordRequest {
    user_id,
    code,
    new_password: new_password.clone(),
  };
  let (status, resp) = ctx.app.api.reset_password(&req).await.unwrap();
  assert_ok!(resp);
  assert!(status.is_success(), "status: {status}");
  let login_req = LoginRequest {
    email: user.email.clone(),
    password: new_password,
  };
  let (status, resp) = ctx.app.api.login(&login_req).await.unwrap();
  let resp = unwrap!(resp);
  assert!(status.is_success(), "status: {status}");
  match resp {
    LoginResponse::Token(token) => {
      assert!(!token.access_token.is_empty());
    }
    LoginResponse::Code { .. } => {
      panic!("Login in reset password test failed.");
    }
  }
}
