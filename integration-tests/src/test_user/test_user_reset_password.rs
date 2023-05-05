use crate::{assert_ok, context::seeder::SeedDbTestContext, unwrap};
use entity::role::RoleUser;
use model::*;
use test_context::test_context;
use util;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_reset_password(ctx: &mut SeedDbTestContext) {
  let user = ctx.users.get(&RoleUser::User).unwrap();
  let new_password: String = util::string::generate_random_string(10);
  let (status, body) = ctx.app.api.forget_password(&user.email).await.unwrap();
  let body = unwrap!(body);
  assert!(status.is_success(), "status: {status}");
  let code = ctx.app.mail.get_code_from_email(&user.email).await.unwrap();
  let req = SetPasswordRequest {
    id: body.id,
    code,
    new_password: new_password.clone(),
  };
  let (status, body) = ctx.app.api.reset_password(&req).await.unwrap();
  assert_ok!(body);
  assert!(status.is_success(), "status: {status}");
  let login_req = LoginRequest::Normal(NormalLogin {
    email: user.email.clone(),
    password: new_password,
  });
  let (status, body) = ctx.app.api.login(&login_req).await.unwrap();
  let body = unwrap!(body);
  assert!(status.is_success(), "status: {status}");
  match body {
    LoginResponse::Token { access_token, .. } => {
      assert!(!access_token.is_empty());
    }
    LoginResponse::Id { id } => {
      panic!("login in reset password test failed: {id:?}");
    }
  }
}
