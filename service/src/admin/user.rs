use client::postgres::PgClient;
use error::AppResult;
use model::*;

pub async fn list(
  pg_client: &PgClient,
  page: PageParamQuery,
) -> AppResult<PageResponse<GetUserResponse>> {
  query::get_transaction(pg_client, move |mut tx| async move {
    let data = query::user::find_page(page.clone())
      .fetch_all(&mut tx)
      .await?;
    let total = query::user::count_all().fetch_one(&mut tx).await?;
    let resp = PageResponse {
      data: data
        .into_iter()
        .map(|user| GetUserResponse {
          id: user.id,
          username: user.username,
          email: user.email,
          role_name: user.role_name,
          is_active: user.is_active,
          is_tfa: user.is_tfa,
          create_at: user.create_at,
        })
        .collect::<Vec<GetUserResponse>>(),
      page_num: page.page_num,
      page_size: page.page_size,
      total: total.total.unwrap(),
    };
    Ok(((resp), tx))
  })
  .await
}
