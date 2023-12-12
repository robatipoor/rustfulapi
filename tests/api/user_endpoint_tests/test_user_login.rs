use crate::assert_ok;
use crate::context::app::AppTestContext;
use crate::context::seeder::SeedDbTestContext;
use crate::unwrap;
use fake::Fake;
use fake::Faker;
use rustfulapi::dto::*;
use rustfulapi::entity::role::RoleUser;
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_login(ctx: &mut SeedDbTestContext) {
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
      assert!(!token.access_token.is_empty());
      assert!(!token.refresh_token.is_empty());
    }
    LoginResponse::Code { .. } => {
      panic!("It was not expected to receive message.");
    }
  }
}

#[test_context(AppTestContext)]
#[tokio::test]
pub async fn test_success_2fa_login(ctx: &mut AppTestContext) {
  let req: RegisterRequest = Faker.fake();
  let (status, resp) = ctx.api.register(&req).await.unwrap();
  assert_ok!(resp);
  assert!(status.is_success());
  let (code, user_id) = ctx
    .mail
    .get_code_and_id_from_email(&req.email)
    .await
    .unwrap();
  let active_req = ActiveRequest {
    user_id,
    code: code.clone(),
  };
  let (status, resp) = ctx.api.active(&active_req).await.unwrap();
  assert_ok!(resp);
  assert!(status.is_success(), "status: {status}");
  let login_req = LoginRequest {
    email: req.email.clone(),
    password: req.password,
  };
  let (status, resp) = ctx.api.login(&login_req).await.unwrap();
  let resp = unwrap!(resp);
  assert!(status.is_success(), "status: {status}");
  match resp {
    LoginResponse::Token(token) => {
      let update_req = UpdateProfileRequest {
        is_2fa: Some(true),
        ..Default::default()
      };
      let (status, _) = ctx
        .api
        .update_profile(&token.access_token, &update_req)
        .await
        .unwrap();
      assert!(status.is_success());
      let (status, resp) = ctx.api.login(&login_req).await.unwrap();
      let resp = unwrap!(resp);
      assert!(status.is_success());
      match resp {
        LoginResponse::Code { message, .. } => {
          assert_eq!(message, "Please check you email.");
          let (code, user_id) = ctx
            .mail
            .get_code_and_id_from_email(&req.email)
            .await
            .unwrap();
          let login_req = Login2faRequest { user_id, code };
          let (status, resp) = ctx.api.login2fa(&login_req).await.unwrap();
          let resp = unwrap!(resp);
          assert!(status.is_success(), "status: {status}");
          match resp {
            LoginResponse::Token(token) => {
              assert!(!token.access_token.is_empty());
            }
            LoginResponse::Code { .. } => {
              panic!("Three login failed.");
            }
          }
        }
        _ => {
          panic!("Second login failed.");
        }
      }
    }
    _ => {
      panic!("First login user failed.");
    }
  }
}
