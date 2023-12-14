use utoipa::{
  openapi::security::{Http, HttpAuthScheme, SecurityScheme},
  Modify, OpenApi,
};

use crate::dto::*;
use crate::entity::role::RoleUser;
use crate::error::{AppError, AppResponseError};
use crate::util::claim::UserClaims;

#[derive(OpenApi)]
#[openapi(
    info(
        version = "v0.1.0",
        title = "RUSTful API",
    ),
    paths(
        // server api 
        crate::handler::server::health_check,
        crate::handler::server::server_state,
        // user api
        crate::handler::user::register,
        crate::handler::user::active,
        crate::handler::user::login,
        crate::handler::user::login2fa,
        crate::handler::user::forget_password,
        crate::handler::user::reset_password,
        crate::handler::user::get_profile,
        crate::handler::user::update_profile,
        crate::handler::user::logout,
        // token api
        crate::handler::token::info,
        crate::handler::token::refresh,
        //admin user api 
        crate::handler::admin::user::list,

    ),
    components(
        schemas(
            RegisterRequest,
            RegisterResponse,
            ActiveRequest,
            ActiveRequest,
            LoginRequest,
            LoginResponse,
            LoginRequest,
            AppResponseError,
            AppError,
            MessageResponse,
            TokenInfoRequest,
            UserClaims,
            ForgetPasswordResponse,
            SetPasswordRequest,
            RegisterResponse,
            TokenResponse,
            ProfileResponse,
            UpdateProfileRequest,
            Direction,
            ServiceStatusResponse,
            GetUserResponse,
            GetUserListResponse,
            RefreshTokenRequest,
            MessageResponse,
            RoleUser,
        )
    ),
    tags(
        (name = "crate::handler::server", description = "server endpoints."),
        (name = "crate::handler::user", description = "user endpoints."),
        (name = "crate::handler::token", description = "token endpoints."),
        (name = "crate::handler::admin", description = "admin endpoints."),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    let components = openapi.components.as_mut().unwrap();
    components.add_security_scheme(
      "jwt",
      SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
    )
  }
}
