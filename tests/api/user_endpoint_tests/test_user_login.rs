use fake::Fake;
use fake::Faker;
use test_context::test_context;

use entity::role::RoleUser;
use model::request::*;
use model::response::*;

use crate::assert_ok;
use crate::context::app::AppTestContext;
use crate::context::seeder::SeedDbTestContext;
use crate::unwrap;

#[test_context(AppTestContext)]
#[tokio::test]
pub async fn test_login_two_factor(ctx: &mut AppTestContext) {
  let req: RegisterRequest = Faker.fake();
  let (status, body) = ctx.api.register(&req).await.unwrap();
  let body = unwrap!(body);
  assert!(status.is_success());
  let code = ctx.mail.get_code_from_email(&req.email).await.unwrap();
  let active_req = ActiveRequest {
    id: body.id,
    code: code.clone(),
  };
  let (status, body) = ctx.api.active(&active_req).await.unwrap();
  assert_ok!(body);
  assert!(status.is_success(), "status: {status}");
  let login_req = LoginRequest::Normal(NormalLogin {
    email: req.email.clone(),
    password: req.password,
  });
  let (status, resp) = ctx.api.login(&login_req).await.unwrap();
  let resp = unwrap!(resp);
  assert!(status.is_success(), "status: {status}");
  match resp {
    LoginResponse::Token { access_token, .. } => {
      let update_req = UpdateProfileRequest {
        is_tfa: Some(true),
        ..Default::default()
      };
      let (status, _body) = ctx
        .api
        .update_profile(&access_token, &update_req)
        .await
        .unwrap();
      assert!(status.is_success());
      let (status, resp) = ctx.api.login(&login_req).await.unwrap();
      let resp = unwrap!(resp);
      assert!(status.is_success());
      match resp {
        LoginResponse::Id { id } => {
          let code = ctx.mail.get_code_from_email(&req.email).await.unwrap();
          let login_req = LoginRequest::TwoFactor(TwoFactorLogin { id, code });
          let (status, resp) = ctx.api.login(&login_req).await.unwrap();
          let resp = unwrap!(resp);
          assert!(status.is_success(), "status: {status}");
          match resp {
            LoginResponse::Token { access_token, .. } => {
              assert!(!access_token.is_empty());
            }
            LoginResponse::Id { .. } => {
              panic!("login2fa_user_test failed1");
            }
          }
        }
        _ => {
          panic!("login2fa_user_test failed2");
        }
      }
    }
    _ => {
      panic!("login2fa_user_test failed3");
    }
  }
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_login_user(ctx: &mut SeedDbTestContext) {
  let user = ctx.users.get(&RoleUser::User).unwrap();
  let req = LoginRequest::Normal(NormalLogin {
    email: user.email.clone(),
    password: user.password.clone(),
  });
  let (status, body) = ctx.app.api.login(&req).await.unwrap();
  let body = unwrap!(body);
  assert!(status.is_success(), "status: {status}");
  match body {
    LoginResponse::Token { access_token, .. } => {
      assert!(!access_token.is_empty());
    }
    LoginResponse::Id { id } => {
      panic!("login_user_test failed: {id}");
    }
  }
}
