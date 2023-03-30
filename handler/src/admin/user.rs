use actix_web::{web, HttpRequest, HttpResponse};
use error::AppResult;
use model::*;
use state::AppState;
use tracing::{info, warn};
use util::claim::UserClaimsRequest;

/// get list users by admin
#[utoipa::path(
    get,
    params(PageParamQuery),
    path = "/api/v1/admin/users",
    responses(
        (status = 200, description = "success get list of users", body = [GetUserResponse]),
        (status = 400, description = "invalid data input", body = [AppResponseError]),
        (status = 500, description = "internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn list(
  state: web::Data<AppState>,
  web::Query(page): web::Query<PageParamQuery>,
  req: HttpRequest,
) -> AppResult<HttpResponse> {
  let admin_id = req.get_user_id()?;
  info!("get list of users by admin_id: {admin_id} page: {page:?}");
  match service::admin::user::list(&state.postgres, page).await {
    Ok(resp) => {
      info!("success get list user: {resp:?}");
      Ok(HttpResponse::Ok().json(resp))
    }
    Err(e) => {
      warn!("unsuccessfully get list user: {e:?}");
      Err(e)
    }
  }
}
