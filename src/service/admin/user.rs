use tracing::info;

use crate::dto::*;
use crate::entity::role::RoleUser;
use crate::error::AppResult;
use crate::repo;
use crate::server::state::AppState;
use crate::util::claim::UserClaims;

// TODO fix order
pub async fn list(
  state: &AppState,
  user: UserClaims,
  param: PageQueryParam,
) -> AppResult<Vec<GetUserResponse>> {
  if user.rol != RoleUser::Admin {
    // TODO
  }
  info!("Get user list with parameter: {param:?}");
  let list = repo::user::find_page(&state.db, param.page_size, param.page_num)
    .await?
    .into_iter()
    .map(|u| GetUserResponse::from(u))
    .collect::<Vec<_>>();
  Ok(list)
}
