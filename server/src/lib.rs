use actix_web::{web, HttpServer};
use configure::AppConfig;
use constant::APP_DOMAIN;
use error::AppResult;
use handler::openapi::API_DOC;
use service::security::cors::get_cors_config;
use state::AppState;
use std::net::TcpListener;
use tracing::info;
use tracing_actix_web::TracingLogger;
use utoipa_swagger_ui::SwaggerUi;

pub mod worker;

pub struct Server {
  pub state: web::Data<AppState>,
  pub listener: TcpListener,
}

impl Server {
  pub async fn new(mut config: AppConfig) -> AppResult<Self> {
    let listener = TcpListener::bind(config.server.get_addr())?;
    config.server.port = listener.local_addr()?.port();
    let state = web::Data::new(AppState::new(config).await?);
    Ok(Self { state, listener })
  }
}

impl Server {
  pub async fn run(self) -> std::io::Result<actix_web::dev::Server> {
    info!(
      "run server: {} profile: {}",
      self.state.config.server.get_addr(),
      self.state.config.profile
    );
    Ok(
      HttpServer::new(move || {
        let state = self.state.clone();
        actix_web::App::new()
          .app_data(state)
          .wrap(service::security::auth::Authentication)
          .wrap(TracingLogger::default())
          .wrap(get_cors_config(
            APP_DOMAIN.to_string(),
            self.state.config.profile,
          ))
          .service(
            SwaggerUi::new("/api/v1/swagger-ui/{_:.*}")
              .url("/api/v1/api-doc/openapi.json", API_DOC.clone()),
          )
          .service(web::scope("/api/v1").configure(router::config))
      })
      .listen(self.listener)?
      .run(),
    )
  }
}
