use axum::extract::{Query, State};
use axum::Json;
use tracing::info;

use crate::error::AppResult;
use crate::server::state::AppState;
use crate::util::claim::UserClaims;
use crate::{dto::*, service};

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
  State(state): State<AppState>,
  user: UserClaims,
  Query(param): Query<PageQueryParam>,
) -> AppResult<Json<Vec<GetUserResponse>>> {
  info!("Get list of user by: {} parameter: {:?}.", user.uid, param);
  match service::admin::user::list(&state, param).await {
    Ok(resp) => {
      info!("Sucess get list of users by user_id: {}.", user.uid);
      Ok(Json(resp))
    }
    Err(e) => {
      info!("Unsuccessful get user list: {e:?}");
      Err(e)
    }
  }
}
