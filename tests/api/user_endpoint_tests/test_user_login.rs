use crate::assert_ok;
use crate::context::app::AppTestContext;
use crate::context::seeder::SeedDbTestContext;
use crate::unwrap;
use fake::Fake;
use fake::Faker;
use rustfulapi::dto::LoginRequest;
use rustfulapi::dto::LoginResponse;
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
    LoginResponse::Message { .. } => {
      panic!("It was not expected to receive message.");
    }
  }
}

// #[test_context(AppTestContext)]
// #[tokio::test]
// pub async fn test_login_two_factor(ctx: &mut AppTestContext) {
//   let req: RegisterRequest = Faker.fake();
//   let (status, body) = ctx.api.register(&req).await.unwrap();
//   let body = unwrap!(body);
//   assert!(status.is_success());
//   let code = ctx.mail.get_code_from_email(&req.email).await.unwrap();
//   let active_req = ActiveRequest {
//     id: body.id,
//     code: code.clone(),
//   };
//   let (status, body) = ctx.api.active(&active_req).await.unwrap();
//   assert_ok!(body);
//   assert!(status.is_success(), "status: {status}");
//   let login_req = LoginRequest::Normal(NormalLogin {
//     email: req.email.clone(),
//     password: req.password,
//   });
//   let (status, resp) = ctx.api.login(&login_req).await.unwrap();
//   let resp = unwrap!(resp);
//   assert!(status.is_success(), "status: {status}");
//   match resp {
//     LoginResponse::Token { access_token, .. } => {
//       let update_req = UpdateProfileRequest {
//         is_2fa: Some(true),
//         ..Default::default()
//       };
//       let (status, _body) = ctx
//         .api
//         .update_profile(&access_token, &update_req)
//         .await
//         .unwrap();
//       assert!(status.is_success());
//       let (status, resp) = ctx.api.login(&login_req).await.unwrap();
//       let resp = unwrap!(resp);
//       assert!(status.is_success());
//       match resp {
//         LoginResponse::Id { id } => {
//           let code = ctx.mail.get_code_from_email(&req.email).await.unwrap();
//           let login_req = LoginRequest::TwoFactor(TwoFactorLogin { id, code });
//           let (status, resp) = ctx.api.login(&login_req).await.unwrap();
//           let resp = unwrap!(resp);
//           assert!(status.is_success(), "status: {status}");
//           match resp {
//             LoginResponse::Token { access_token, .. } => {
//               assert!(!access_token.is_empty());
//             }
//             LoginResponse::Id { .. } => {
//               panic!("login2fa_user_test failed1");
//             }
//           }
//         }
//         _ => {
//           panic!("login2fa_user_test failed2");
//         }
//       }
//     }
//     _ => {
//       panic!("login2fa_user_test failed3");
//     }
//   }
// }
