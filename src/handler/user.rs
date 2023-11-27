use axum::extract::{Query, Request, State};
use axum::response::Response;
use axum::Json;
use garde::Validate;
use tracing::{info, warn};

use crate::error::AppResult;
use crate::server::state::AppState;
use crate::util::claim::{UserClaims, UserClaimsRequest};
use crate::{dto::*, service};

/// register new user
#[utoipa::path(
    post,
    request_body = RegisterRequest,
    path = "/api/v1/users/register",
    responses(
        (status = 200, description = "success register user", body = [RegisterResponse]),
        (status = 400, description = "invalid data input", body = [AppResponseError]),
        (status = 500, description = "internal server error", body = [AppResponseError])
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
// /// get invitation token registered user
// #[utoipa::path(
//     put,
//     request_body = InvitationRequest,
//     path = "/api/v1/users/invitation",
//     responses(
//         (status = 200, description = "success get invitation token", body = [InvitationResponse]),
//         (status = 400, description = "invalid data input", body = [AppResponseError]),
//         (status = 500, description = "internal server error", body = [AppResponseError])
//     )
// )]
// pub async fn invitation(
//   State(_state): State<AppState>,
//   Json(req): Json<InvitationRequest>,
// ) -> AppResult<Json<InvitationResponse>> {
//   info!("invitation request user: {req:?}");
//   // match service::user::invitation(&state, req).await {
//   //   Ok(resp) => {
//   //     info!("success invitation token send: {resp:?}");
//   //     Ok(HttpResponse::Ok().json(resp))
//   //   }
//   //   Err(e) => {
//   //     warn!("unsuccessfully get invitation token error: {e:?}",);
//   //     Err(e)
//   //   }
//   // }
//   todo!()
// }
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
  State(_state): State<AppState>,
  Json(req): Json<ActiveRequest>,
) -> AppResult<Response> {
  info!("active user with token: {req:?}");
  // match service::user::active(&state, req).await {
  //   Ok(_) => {
  //     info!("success active user");
  //     Ok(HttpResponse::Ok().json(MessageResponse::new("user activated")))
  //   }
  //   Err(e) => {
  //     info!("unsuccessfully active user: {e:?}");
  //     Err(e)
  //   }
  // }

  todo!()
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
  State(_state): State<AppState>,
  Json(_body): Json<ValidateRequest>,
  req: Request,
) -> AppResult<Response> {
  let user_id = req.get_user_id()?;
  info!("get validate token user_id: {user_id}");
  // match service::user::validate(&state, &user_id, validate_req).await {
  //   Ok(resp) => {
  //     info!("success validate token user_id: {user_id} resp: {resp:?}");
  //     Ok(HttpResponse::Ok().json(resp))
  //   }
  //   Err(e) => {
  //     warn!("unsuccessfully validate token user_id: {user_id} error: {e:?}");
  //     Err(e)
  //   }
  // }
  todo!()
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
  State(_state): State<AppState>,
  Json(req): Json<LoginRequest>,
) -> AppResult<Response> {
  info!("login user with request req: {req:?}");
  // match service::user::login(&state, req).await {
  //   Ok(resp) => {
  //     info!("success login user_id: {resp:?}");
  //     Ok(HttpResponse::Ok().json(resp))
  //   }
  //   Err(e) => {
  //     warn!("unsuccessfully login user error: {e:?}");
  //     Err(e)
  //   }
  // }
  todo!()
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
  State(_state): State<AppState>,
  user: UserClaims,
) -> AppResult<Response> {
  info!("refresh token with claims: {user:?}");
  // match service::user::refresh_token(&state, &claims).await {
  //   Ok(resp) => {
  //     info!("success refresh token user resp: {resp:?}");
  //     Ok(HttpResponse::Ok().json(resp))
  //   }
  //   Err(e) => {
  //     warn!("unsuccessfully refresh token error: {e:?}");
  //     Err(e)
  //   }
  // }
  todo!()
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
pub async fn logout(State(_state): State<AppState>, user: UserClaims) -> AppResult<Response> {
  info!("logout user_id: {}", user.uid);
  // match service::user::logout(&state, user_id).await {
  //   Ok(_) => {
  //     info!("success logout user user_id: {user_id}");
  //     Ok(HttpResponse::Ok().json(MessageResponse::new("the user is logged out successfully")))
  //   }
  //   Err(e) => {
  //     warn!("unsuccessfully logout user: {e:?}");
  //     Err(e)
  //   }
  // }
  todo!()
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
  State(_state): State<AppState>,
  query: Query<ForgetPasswordParamQuery>,
) -> AppResult<Response> {
  info!("forget password user query: {:?}", query.0);
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
  State(_state): State<AppState>,
  Json(body): Json<SetPasswordRequest>,
) -> AppResult<Response> {
  info!("set password user request: {body:?}");
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
pub async fn get_profile(State(_state): State<AppState>, user: UserClaims) -> AppResult<Response> {
  info!("get profile user id: {}", user.uid);
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
  State(_state): State<AppState>,
  user: UserClaims,
  Json(_body): Json<UpdateProfileRequest>,
) -> AppResult<Response> {
  info!("update profile user_id: {}", user.uid);
  // match service::user::update_profile(&state, user_id, body).await {
  //   Ok(_) => {
  //     info!("success update profile user user_id: {user_id}");
  //     Ok(HttpResponse::Ok().json(MessageResponse::new("user profile updated")))
  //   }
  //   Err(e) => {
  //     info!("unsuccessful update profile user: {e:?}");
  //     Err(e)
  //   }
  // }

  todo!()
}
