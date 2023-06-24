use actix_web::{web, HttpRequest, HttpResponse};
use tracing::{info, warn};

use error::AppResult;
use model::response::MessageResponse;
use model::*;
use state::AppState;
use util::claim::UserClaimsRequest;

/// register new user
#[utoipa::path(
    post,
    request_body = RegisterRequest,
    path = "/api/v1/users/register",
    responses(
        (status = 201, description = "success register user", body = [RegisterResponse]),
        (status = 400, description = "invalid data input", body = [AppResponseError]),
        (status = 500, description = "internal server error", body = [AppResponseError])
    )
)]
pub async fn register(
  state: web::Data<AppState>,
  web::Json(req): web::Json<RegisterRequest>,
) -> AppResult<HttpResponse> {
  info!("register user with request: {req:?}");
  match service::user::register(state, req).await {
    Ok((user_id, resp)) => {
      info!("success register user: {user_id}");
      Ok(HttpResponse::Created().json(resp))
    }
    Err(e) => {
      warn!("unsuccessfully register user: {e:?}");
      Err(e)
    }
  }
}
/// get invitation token registered user
#[utoipa::path(
    put,
    request_body = InvitationRequest,
    path = "/api/v1/users/invitation",
    responses(
        (status = 200, description = "success get invitation token", body = [InvitationResponse]),
        (status = 400, description = "invalid data input", body = [AppResponseError]),
        (status = 500, description = "internal server error", body = [AppResponseError])
    )
)]
pub async fn invitation(
  state: web::Data<AppState>,
  web::Json(req): web::Json<InvitationRequest>,
) -> AppResult<HttpResponse> {
  info!("invitation request user: {req:?}");
  match service::user::invitation(&state, req).await {
    Ok(resp) => {
      info!("success invitation token send: {resp:?}");
      Ok(HttpResponse::Ok().json(resp))
    }
    Err(e) => {
      warn!("unsuccessfully get invitation token error: {e:?}",);
      Err(e)
    }
  }
}
/// active registered user
#[utoipa::path(
    put,
    request_body = ActiveRequest,
    path = "/api/v1/users/active",
    responses(
        (status = 200, description = "success active user", body = [MessageResponse]),
        (status = 400, description = "invalid data input", body = [AppResponseError]),
        (status = 500, description = "internal server error", body = [AppResponseError])
    )
)]
pub async fn active(
  state: web::Data<AppState>,
  web::Json(req): web::Json<ActiveRequest>,
) -> AppResult<HttpResponse> {
  info!("active user with token: {req:?}");
  match service::user::active(&state, req).await {
    Ok(_) => {
      info!("success active user");
      Ok(HttpResponse::Ok().json(MessageResponse::new("user activated")))
    }
    Err(e) => {
      info!("unsuccessfully active user: {e:?}");
      Err(e)
    }
  }
}
/// validate user token
#[utoipa::path(
    post,
    path = "/api/v1/users/validate",
    request_body = ValidateRequest,
    responses(
        (status = 200, description = "token is valid", body = [UserClaims]),
        (status = 400, description = "invalid token", body = [AppResponseError]),
        (status = 401, description = "unauthorized user", body = [AppResponseError]),
        (status = 500, description = "internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn validate(
  state: web::Data<AppState>,
  web::Json(validate_req): web::Json<ValidateRequest>,
  req: HttpRequest,
) -> AppResult<HttpResponse> {
  let user_id = req.get_user_id()?;
  info!("get validate token user_id: {user_id}");
  match service::user::validate(&state, &user_id, validate_req).await {
    Ok(resp) => {
      info!("success validate token user_id: {user_id} resp: {resp:?}");
      Ok(HttpResponse::Ok().json(resp))
    }
    Err(e) => {
      warn!("unsuccessfully validate token user_id: {user_id} error: {e:?}");
      Err(e)
    }
  }
}
/// normal login & two factor login user
#[utoipa::path(
    post,
    request_body = LoginRequest,
    path = "/api/v1/users/login",
    responses(
        (status = 200, description = "success login user", body = [LoginResponse]),
        (status = 400, description = "invalid data input", body = [AppResponseError]),
        (status = 404, description = "user not found", body = [AppResponseError]),
        (status = 500, description = "internal server error", body = [AppResponseError])
    )
)]
pub async fn login(
  state: web::Data<AppState>,
  web::Json(req): web::Json<LoginRequest>,
) -> AppResult<HttpResponse> {
  info!("login user with request req: {req:?}");
  match service::user::login(&state, req).await {
    Ok(resp) => {
      info!("success login user_id: {resp:?}");
      Ok(HttpResponse::Ok().json(resp))
    }
    Err(e) => {
      warn!("unsuccessfully login user error: {e:?}");
      Err(e)
    }
  }
}

/// get new tokens
#[utoipa::path(
    get,
    path = "/api/v1/users/token",
    responses(
        (status = 200, description = "success get new access token and refresh token", body = [TokenResponse]),
        (status = 400, description = "invalid data input", body = [AppResponseError]),
        (status = 401, description = "unauthorized user", body = [AppResponseError]),
        (status = 500, description = "internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn refresh_token(
  state: web::Data<AppState>,
  req: HttpRequest,
) -> AppResult<HttpResponse> {
  let claims = req.get_user_claims()?;
  info!("refresh token with claims: {claims:?}");
  match service::user::refresh_token(&state, &claims).await {
    Ok(resp) => {
      info!("success refresh token user resp: {resp:?}");
      Ok(HttpResponse::Ok().json(resp))
    }
    Err(e) => {
      warn!("unsuccessfully refresh token error: {e:?}");
      Err(e)
    }
  }
}

/// logout user
#[utoipa::path(
    get,
    path = "/api/v1/users/logout",
    responses(
        (status = 200, description = "success logout user", body = [MessageResponse]),
        (status = 401, description = "unauthorized user", body = [AppResponseError]),
        (status = 500, description = "internal error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn logout(state: web::Data<AppState>, req: HttpRequest) -> AppResult<HttpResponse> {
  let user_id = req.get_user_id()?;
  info!("logout user_id: {user_id}");
  match service::user::logout(&state, user_id).await {
    Ok(_) => {
      info!("success logout user user_id: {user_id}");
      Ok(HttpResponse::Ok().json(MessageResponse::new("the user is logged out successfully")))
    }
    Err(e) => {
      warn!("unsuccessfully logout user: {e:?}");
      Err(e)
    }
  }
}

/// forgot user password
#[utoipa::path(
    get,
    path = "/api/v1/users/password",
    params(ForgetPasswordParamQuery),
    responses(
        (
            status = 200,
            description = "success send mail reset password code",
            body = [ForgetPasswordResponse],
        ),
        (status = 404, description = "user not found", body = [AppResponseError]),
        (status = 500, description = "internal error", body = [AppResponseError])
    )
)]
pub async fn forget_password(
  state: web::Data<AppState>,
  query: web::Query<ForgetPasswordParamQuery>,
) -> AppResult<HttpResponse> {
  info!("forget password user query: {:?}", query.0);
  match service::user::forget_password(&state, query.0).await {
    Ok(resp) => {
      info!("success forget password user response");
      Ok(HttpResponse::Created().json(resp))
    }
    Err(e) => {
      warn!("unsuccessful forget password user: {e:?}");
      Err(e)
    }
  }
}
/// reset user password
#[utoipa::path(
    put,
    path = "/api/v1/users/password",
    request_body = SetPasswordRequest,
    responses(
        (status = 200, description = "success update password login" , body = [MessageResponse]),
        (status = 400, description = "invalid data input", body = [AppResponseError]),
        (status = 500, description = "internal error", body = [AppResponseError])
    )
)]
pub async fn reset_password(
  state: web::Data<AppState>,
  body: web::Json<SetPasswordRequest>,
) -> AppResult<HttpResponse> {
  let req = body.into_inner();
  info!("set password user request: {req:?}");
  match service::user::reset_password(&state, req).await {
    Ok(_) => {
      info!("success set new password");
      Ok(HttpResponse::Ok().json(MessageResponse::new("the password has been updated")))
    }
    Err(e) => {
      warn!("unsuccessful set password user: {e:?}");
      Err(e)
    }
  }
}

/// get user profile information
#[utoipa::path(
    get,
    path = "/api/v1/users/profile",
    responses(
        (status = 200, description = "success get user profile", body = [ProfileResponse]),
        (status = 401, description = "unauthorized user", body = [AppResponseError]),
        (status = 500, description = "internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn get_profile(state: web::Data<AppState>, req: HttpRequest) -> AppResult<HttpResponse> {
  let user_id = req.get_user_id()?;
  info!("get profile user id: {user_id:?}");
  match service::user::get_profile(&state, &user_id).await {
    Ok(resp) => {
      info!("success get profile user: {user_id}");
      Ok(HttpResponse::Ok().json(resp))
    }
    Err(e) => {
      warn!("unsuccessfully get profile user: {e:?}");
      Err(e)
    }
  }
}

/// update profile information
#[utoipa::path(
    put,
    path = "/api/v1/users/profile",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "success update profile information", body = [MessageResponse]),
        (status = 400, description = "invalid data input", body = [AppResponseError]),
        (status = 401, description = "unauthorized user", body = [AppResponseError]),
        (status = 500, description = "internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn update_profile(
  state: web::Data<AppState>,
  req: HttpRequest,
  web::Json(body): web::Json<UpdateProfileRequest>,
) -> AppResult<HttpResponse> {
  let user_id = req.get_user_id()?;
  info!("update profile user_id: {user_id}");
  match service::user::update_profile(&state, user_id, body).await {
    Ok(_) => {
      info!("success update profile user user_id: {user_id}");
      Ok(HttpResponse::Ok().json(MessageResponse::new("user profile updated")))
    }
    Err(e) => {
      info!("unsuccessful update profile user: {e:?}");
      Err(e)
    }
  }
}
