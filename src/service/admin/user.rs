use tracing::info;

use crate::dto::*;
use crate::entity::role::RoleUser;
use crate::error::{AppError, AppResult};
use crate::repo;
use crate::server::state::AppState;
use crate::util::claim::UserClaims;

pub async fn list(
  state: &AppState,
  user: &UserClaims,
  param: PageQueryParam,
) -> AppResult<GetUserListResponse> {
  if user.rol != RoleUser::Admin {
    return Err(AppError::UnauthorizedError(
      "This user is not authorized to use this resource.".to_string(),
    ));
  }
  info!("Get user list with parameter: {param:?}");
  let list = repo::user::find_page(&state.db, param)
    .await?
    .into_iter()
    .map(|u| GetUserResponse::from(u))
    .collect::<Vec<_>>();
  Ok(GetUserListResponse { list })
}
