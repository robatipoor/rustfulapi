use sqlx::postgres::{PgArguments, PgRow};

pub trait BaseQuery<'a> {
  type Id;
  type Entity;

  fn find_by_id<F>(id: Self::Id) -> sqlx::query::Map<'static, sqlx::Postgres, F, PgArguments>
  where
    F: FnMut(PgRow) -> Result<Self::Entity, sqlx::Error>;

  fn find_first<F>() -> sqlx::query::Map<'static, sqlx::Postgres, F, PgArguments>
  where
    F: FnMut(PgRow) -> Result<Self::Entity, sqlx::Error>;

  fn find_all<F>() -> sqlx::query::Map<'static, sqlx::Postgres, F, PgArguments>
  where
    F: FnMut(PgRow) -> Result<Self::Entity, sqlx::Error>;

  fn find_page<F>(
    page: model::request::PageParamQuery,
  ) -> sqlx::query::Map<'static, sqlx::Postgres, F, PgArguments>
  where
    F: FnMut(PgRow) -> Result<Self::Entity, sqlx::Error>;

  fn count_all<F>() -> sqlx::query::Map<'static, sqlx::Postgres, F, PgArguments>
  where
    F: FnMut(PgRow) -> Result<model::record::TotalRecord, sqlx::Error>;

  fn exist_by_id<F>(id: Self::Id) -> sqlx::query::Map<'static, sqlx::Postgres, F, PgArguments>
  where
    F: FnMut(PgRow) -> Result<model::record::ExistRecord, sqlx::Error>;

  fn save(&'a self) -> sqlx::query::Query<'a, sqlx::Postgres, PgArguments>;

  fn update(&'a self) -> sqlx::query::Query<'a, sqlx::Postgres, PgArguments>;

  fn delete_by_id(id: Self::Id) -> sqlx::query::Query<'a, sqlx::Postgres, PgArguments>;
}

// #[derive(BaseQuery)]
// #[base_query(table_name = "test")]
// pub struct Test {
//     pub id: u32,
//     pub name: String,
//     pub age: u32,
// }
