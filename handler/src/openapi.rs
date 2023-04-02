use once_cell::sync::Lazy;
use utoipa::{
  openapi::security::{Http, HttpAuthScheme, SecurityScheme},
  Modify, OpenApi,
};

use entity::*;
use error::{AppError, AppResponseError};
use model::*;
use util::claim::UserClaims;

pub static API_DOC: Lazy<utoipa::openapi::OpenApi> = Lazy::new(ApiDoc::openapi);

#[derive(OpenApi)]
#[openapi(
    info(
        version = "v0.1.0",
        title = "RUSTful APIs",
    ),
    paths(
        // server api 
        crate::server::health_check,
        crate::server::server_state,
        // user api
        crate::user::register,
        crate::user::invitation,
        crate::user::active,
        crate::user::validate,
        crate::user::refresh_token,
        crate::user::login,
        crate::user::forget_password,
        crate::user::reset_password,
        crate::user::get_profile,
        crate::user::update_profile,
        crate::user::logout,
        //admin user api 
        crate::admin::user::list,

    ),
    components(
        schemas(
            RoleUser,
            RegisterRequest,
            RegisterResponse,
            InvitationRequest,
            InvitationResponse,
            ActiveRequest,
            LoginRequest,
            TwoFactorLogin,
            NormalLogin,
            LoginResponse,
            TwoFactorLoginRequest,
            AppResponseError,
            AppError,
            MessageResponse,
            ValidateRequest,
            UserClaims,
            ForgetPasswordResponse,
            SetPasswordRequest,
            RegisterResponse,
            TokenResponse,
            PageParamQuery,
            ProfileResponse,
            UpdateProfileRequest,
            Direction,
            ServiceStatusResponse,
            GetUserResponse,
        )
    ),
    tags(
        (name = "crate::server", description = "server endpoints."),
        (name = "crate::user", description = "user endpoints."),
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
