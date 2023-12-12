use entity::role::RoleUser;
use entity::user::User;
use fake::{Fake, Faker};
use model::*;
use reqwest::StatusCode;
use service;
use test_context::test_context;

use crate::{assert_err, assert_ok, context::app::AppTestContext};

#[test_context(AppTestContext)]
#[tokio::test]
pub async fn test_validate_token(ctx: &mut AppTestContext) {
  let mut user: User = Faker.fake();
  user.role_name = RoleUser::System;
  let (claims, _token_response) =
    service::token::generate_token_response(&ctx.state.redis, &ctx.state.config.secret, 60, &user)
      .await
      .unwrap();
  let access_token =
    service::token::encode_access_token(&ctx.state.config.secret, &claims).unwrap();
  let req = ValidateRequest {
    token: access_token.clone(),
  };
  let (status, body) = ctx.api.validate(&access_token, &req).await.unwrap();
  assert_ok!(body);
  assert!(status.is_success(), "status: {status}");
}

#[test_context(AppTestContext)]
#[tokio::test]
pub async fn test_validate_token_with_invalid_access(ctx: &mut AppTestContext) {
  let mut user: User = Faker.fake();
  user.role_name = RoleUser::User;
  let (claims, _token_response) =
    service::token::generate_token_response(&ctx.state.redis, &ctx.state.config.secret, 60, &user)
      .await
      .unwrap();
  let token = service::token::encode_access_token(&ctx.state.config.secret, &claims).unwrap();
  let req = ValidateRequest {
    token: token.clone(),
  };
  let (status, body) = ctx.api.validate(&token, &req).await.unwrap();
  assert_err!(body);
  assert_eq!(status, StatusCode::FORBIDDEN);
}
