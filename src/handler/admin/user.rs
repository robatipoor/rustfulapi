use axum::extract::State;
use axum::Json;

use crate::dto::*;
use crate::error::AppResult;
use crate::server::state::AppState;
use crate::util::claim::UserClaims;

/// Get list of user .
#[utoipa::path(
    put,
    path = "/api/v1/admin/user/list",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Success get list of users", body = [MessageResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn list(
  State(_state): State<AppState>,
  _user: UserClaims,
) -> AppResult<Json<MessageResponse>> {
  // info!("Update profile user_id: {}.", user.uid);
  // match service::user::update_profile(&state, user.uid, req).await {
  //   Ok(_) => {
  //     info!("Success update profile user user_id: {}.", user.uid);
  //     Ok(Json(MessageResponse::new("User profile updated.")))
  //   }
  //   Err(e) => {
  //     info!("Unsuccessful update profile user: {e:?}");
  //     Err(e)
  //   }
  // }
  todo!()
}
