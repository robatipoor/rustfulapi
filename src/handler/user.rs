use axum::extract::{Query, State};
use axum::Json;
use garde::Validate;
use tracing::{info, warn};

use crate::error::AppResult;
use crate::server::state::AppState;
use crate::util::claim::UserClaims;
use crate::{dto::*, service};

/// Register new user.
#[utoipa::path(
    post,
    request_body = RegisterRequest,
    path = "/api/v1/users/register",
    responses(
        (status = 200, description = "Success register user", body = [RegisterResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn register(
  State(state): State<AppState>,
  Json(req): Json<RegisterRequest>,
) -> AppResult<Json<RegisterResponse>> {
  info!("Register new user with request: {req:?}");
  req.validate(&())?;
  match service::user::register(state, req).await {
    Ok(user_id) => {
      info!("Successfully register user: {user_id}");
      let resp = RegisterResponse { id: user_id };
      Ok(Json(resp))
    }
    Err(e) => {
      warn!("Unsuccessfully register user: {e:?}");
      Err(e)
    }
  }
}

/// Active registered user.
#[utoipa::path(
    put,
    request_body = ActiveRequest,
    path = "/api/v1/users/active",
    responses(
        (status = 200, description = "Success active user", body = [MessageResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn active(
  State(state): State<AppState>,
  Json(req): Json<ActiveRequest>,
) -> AppResult<Json<MessageResponse>> {
  info!("Active user with token: {req:?}.");
  match service::user::active(&state, req).await {
    Ok(_) => {
      info!("User successfully activated.");
      Ok(Json(MessageResponse::new("User successfully activated.")))
    }
    Err(e) => {
      info!("The user activation operation was not successful: {e:?}");
      Err(e)
    }
  }
}

/// Login user.
#[utoipa::path(
    post,
    request_body = LoginRequest,
    path = "/api/v1/users/login",
    responses(
        (status = 200, description = "Success login user", body = [LoginResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 404, description = "User not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn login(
  State(state): State<AppState>,
  Json(req): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
  info!("Login user with request: {req:?}.");
  match service::user::login(&state, req).await {
    Ok(resp) => {
      info!("Success login user_id: {resp:?}.");
      Ok(Json(resp))
    }
    Err(e) => {
      warn!("Unsuccessfully login user error: {e:?}.");
      Err(e)
    }
  }
}

/// Login2fa user.
#[utoipa::path(
    post,
    request_body = LoginRequest,
    path = "/api/v1/users/login2fa",
    responses(
        (status = 200, description = "Success login user", body = [LoginResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 404, description = "User not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn login2fa(
  State(state): State<AppState>,
  Json(req): Json<Login2faRequest>,
) -> AppResult<Json<LoginResponse>> {
  info!("Two factor login user with request: {req:?}.");
  match service::user::login2fa(&state, req).await {
    Ok(resp) => {
      info!("Success login user_id: {resp:?}.");
      Ok(Json(LoginResponse::Token(resp)))
    }
    Err(e) => {
      warn!("Unsuccessfully login user error: {e:?}.");
      Err(e)
    }
  }
}

/// Logout user.
#[utoipa::path(
    get,
    path = "/api/v1/users/logout",
    responses(
        (status = 200, description = "Success logout user", body = [MessageResponse]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn logout(
  State(state): State<AppState>,
  user: UserClaims,
) -> AppResult<Json<MessageResponse>> {
  info!("Logout user_id: {}", user.uid);
  match service::user::logout(&state, user.uid).await {
    Ok(_) => {
      info!("Success logout user user_id: {}", user.uid);
      Ok(Json(MessageResponse::new(
        "This user has successfully logged out.",
      )))
    }
    Err(e) => {
      warn!("unsuccessfully logout user: {e:?}");
      Err(e)
    }
  }
}

/// Forgot user password.
#[utoipa::path(
    get,
    path = "/api/v1/users/password",
    params(ForgetPasswordQueryParam),
    responses(
        (
            status = 200,
            description = "Success send mail reset password code",
            body = [ForgetPasswordResponse],
        ),
        (status = 404, description = "User not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn forget_password(
  State(_state): State<AppState>,
  Query(param): Query<ForgetPasswordQueryParam>,
) -> AppResult<Json<ForgetPasswordResponse>> {
  info!("Forget password user query parameter: {param:?}");
  // match service::user::forget_password(&state, query.0).await {
  //   Ok(resp) => {
  //     info!("success forget password user response");
  //     Ok(HttpResponse::Created().json(resp))
  //   }
  //   Err(e) => {
  //     warn!("unsuccessful forget password user: {e:?}");
  //     Err(e)
  //   }
  // }
  todo!()
}

/// Reset user password.
#[utoipa::path(
    put,
    path = "/api/v1/users/password",
    request_body = SetPasswordRequest,
    responses(
        (status = 200, description = "Success update password login" , body = [MessageResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn reset_password(
  State(_state): State<AppState>,
  Json(req): Json<SetPasswordRequest>,
) -> AppResult<Json<MessageResponse>> {
  info!("Reset password user request: {req:?}.");
  // match service::user::reset_password(&state, req).await {
  //   Ok(_) => {
  //     info!("success set new password");
  //     Ok(HttpResponse::Ok().json(MessageResponse::new("the password has been updated")))
  //   }
  //   Err(e) => {
  //     warn!("unsuccessful set password user: {e:?}");
  //     Err(e)
  //   }
  // }
  todo!()
}

/// Get user profile information.
#[utoipa::path(
    get,
    path = "/api/v1/users/profile",
    responses(
        (status = 200, description = "Success get user profile", body = [ProfileResponse]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn get_profile(
  State(_state): State<AppState>,
  user: UserClaims,
) -> AppResult<Json<ProfileResponse>> {
  info!("Get profile user id: {}.", user.uid);
  // match service::user::get_profile(&state, &user_id).await {
  //   Ok(resp) => {
  //     info!("success get profile user: {user_id}");
  //     Ok(HttpResponse::Ok().json(resp))
  //   }
  //   Err(e) => {
  //     warn!("unsuccessfully get profile user: {e:?}");
  //     Err(e)
  //   }
  // }
  todo!()
}

/// Update user profile.
#[utoipa::path(
    put,
    path = "/api/v1/users/profile",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Success update profile information", body = [MessageResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn update_profile(
  State(state): State<AppState>,
  user: UserClaims,
  Json(req): Json<UpdateProfileRequest>,
) -> AppResult<Json<MessageResponse>> {
  info!("Update profile user_id: {}.", user.uid);
  match service::user::update_profile(&state, user.uid, req).await {
    Ok(_) => {
      info!("Success update profile user user_id: {}", user.uid);
      Ok(Json(MessageResponse::new("User profile updated.")))
    }
    Err(e) => {
      info!("Unsuccessful update profile user: {e:?}");
      Err(e)
    }
  }
}
