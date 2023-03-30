use std::rc::Rc;
use std::{
  pin::Pin,
  task::{Context, Poll},
};

use actix_service::{Service, Transform};
use actix_web::body::{EitherBody, MessageBody};
use actix_web::{
  dev::{ServiceRequest, ServiceResponse},
  Error,
};
use actix_web::{http, web, HttpMessage};
use actix_web_grants::permissions::AttachPermissions;
use anyhow::anyhow;
use futures::Future;
use tracing::{error, info};

use constant::IGNORE_ROUTES;
use error::AppError;
use state::AppState;
use util::claim::parse_bearer_token_from_header;

pub struct AuthenticationMiddleware<S> {
  service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  S::Future: 'static,
  B: MessageBody + 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

  fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(cx)
  }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    info!("auth middleware path: {}", req.path());
    let service = Rc::clone(&self.service);
    Box::pin(async move {
      if req.method() == http::Method::OPTIONS {
        return service.call(req).await.map(|res| res.map_into_left_body());
      }
      for ignore_route in IGNORE_ROUTES.iter() {
        if req.path().starts_with(ignore_route) {
          return service.call(req).await.map(|res| res.map_into_left_body());
        }
      }
      let state = match req.app_data::<web::Data<AppState>>() {
        Some(state) => state,
        None => {
          error!("get app state failed");
          let err = AppError::UnknownError(anyhow!("Get Data State From Request Failed"));
          return Ok(
            req
              .error_response::<actix_web::Error>(err.into())
              .map_into_right_body(),
          );
        }
      };
      let token = match parse_bearer_token_from_header(req.headers()) {
        Ok(token) => token,
        Err(e) => {
          info!("parse token failed path: {} error: {e}", req.path());
          let err = AppError::Unauthorized("User Not Authorized".to_string());
          return Ok(
            req
              .error_response::<actix_web::Error>(err.into())
              .map_into_right_body(),
          );
        }
      };
      let req =
        match crate::token::verify_token(&state.redis, &state.config.secret, &token, req.path())
          .await
        {
          Ok(token_data) => {
            req.attach(vec![token_data.claims.rol.to_string()]);
            req.extensions_mut().insert(token_data.claims);
            req
          }
          Err(err) => {
            return Ok(req.error_response(err).map_into_right_body());
          }
        };
      service.call(req).await.map(|res| res.map_into_left_body())
    })
  }
}

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  S::Future: 'static,
  B: MessageBody + 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type Transform = AuthenticationMiddleware<S>;
  type InitError = ();
  type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    futures::future::ok(AuthenticationMiddleware {
      service: Rc::new(service),
    })
  }
}

#[cfg(test)]
mod tests {

  use actix_web::{
    dev::Service, http, http::StatusCode, test::TestRequest, web, App, HttpResponse,
  };
  use configure::CONFIG;
  use entity::user::User;
  use fake::{Fake, Faker};
  use test_context::{test_context, AsyncTestContext};

  use super::*;

  #[allow(dead_code)]
  struct AuthContextTest {
    state: web::Data<AppState>,
  }

  #[async_trait::async_trait]
  impl AsyncTestContext for AuthContextTest {
    async fn setup() -> Self {
      let state = web::Data::new(AppState::new(CONFIG.clone()).await.unwrap());
      Self { state }
    }

    async fn teardown(self) {}
  }

  #[test_context(AuthContextTest)]
  #[tokio::test]
  async fn test_middleware_unauthorized(ctx: &mut AuthContextTest) {
    let req = TestRequest::with_uri("/api/v1/user/logout").to_request();
    let srv = actix_web::test::init_service(
      App::new()
        .wrap(Authentication)
        .app_data(ctx.state.clone())
        .route("/", web::get().to(HttpResponse::Ok)),
    )
    .await;
    let resp = srv.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
  }

  #[test_context(AuthContextTest)]
  #[tokio::test]
  async fn test_middleware_authorized(ctx: &mut AuthContextTest) {
    let req = TestRequest::with_uri("/api/v1/users/login").to_request();
    let srv = actix_web::test::init_service(
      App::new()
        .wrap(Authentication)
        .app_data(ctx.state.clone())
        .route("/api/v1/users/login", web::get().to(HttpResponse::Ok)),
    )
    .await;
    let resp = srv.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
  }

  #[test_context(AuthContextTest)]
  #[tokio::test]
  async fn test_middleware_with_authorization_header(ctx: &mut AuthContextTest) {
    let user: User = Faker.fake();
    let (claims, _) =
      crate::token::generate_token_response(&ctx.state.redis, &ctx.state.config.secret, 10, &user)
        .await
        .unwrap();
    let token = crate::token::encode_access_token(&ctx.state.config.secret, &claims).unwrap();
    let req = TestRequest::with_uri("/api/v1/user/logout")
      .append_header((http::header::AUTHORIZATION, format!("Bearer {token}")))
      .to_request();
    let srv = actix_web::test::init_service(
      App::new()
        .wrap(Authentication)
        .app_data(ctx.state.clone())
        .route("/api/v1/user/logout", web::get().to(HttpResponse::Ok)),
    )
    .await;
    let resp = srv.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
  }
}
