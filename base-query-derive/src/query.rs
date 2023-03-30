use crate::table::Table;
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_base_query(table: &Table) -> TokenStream {
  let table_ident = &table.ident;
  // let id_ident = &table.id.field;
  let id_ty = &table.id.ty;
  let find_by_id = find_by_id(table);
  let find_first = find_first(table);
  let find_all = find_all(table);
  let find_page = find_page(table);
  let count_all = count_all(table);
  let exist_by_id = exist_by_id(table);
  let save = save(table);
  let update = update(table);
  let delete_by_id = delete_by_id(table);

  quote! {
      impl<'a> base_query::BaseQuery<'a> for #table_ident {
          type Id = #id_ty;
          type Entity = #table_ident;

          #find_by_id
          #find_first
          #find_all
          #find_page
          #count_all
          #exist_by_id
          #save
          #update
          #delete_by_id
      }
  }
}

fn find_by_id(_table: &Table) -> TokenStream {
  quote! {
  fn find_by_id<F>(id: Self::Id) -> sqlx::query::Map<'static, sqlx::Postgres, F, PgArguments>
  where
      F: FnMut(PgRow) -> Result<Self::Entity, sqlx::Error>{
          todo!();
      }
  }
}

fn find_first(_table: &Table) -> TokenStream {
  quote! {
  fn find_first<F>() -> sqlx::query::Map<'static, sqlx::Postgres, F, PgArguments>
  where
      F: FnMut(PgRow) -> Result<Self::Entity, sqlx::Error>{
          todo!();
      }
  }
}

fn find_all(_table: &Table) -> TokenStream {
  quote! {
  fn find_all<F>() -> sqlx::query::Map<'static, sqlx::Postgres, F, PgArguments>
  where
      F: FnMut(PgRow) -> Result<Self::Entity, sqlx::Error>{
          todo!();
      }
  }
}

fn find_page(_table: &Table) -> TokenStream {
  quote! {
  fn find_page<F>(
      page: model::request::PageParamQuery,
  ) -> sqlx::query::Map<'static, sqlx::Postgres, F, PgArguments>
  where
      F: FnMut(PgRow) -> Result<Self::Entity, sqlx::Error>{
          todo!();
      }
  }
}

fn count_all(_table: &Table) -> TokenStream {
  quote! {
  fn count_all<F>() -> sqlx::query::Map<'static, sqlx::Postgres, F, PgArguments>
  where
      F: FnMut(PgRow) -> Result<model::record::TotalRecord, sqlx::Error>{
          todo!()
      }
  }
}

fn exist_by_id(_table: &Table) -> TokenStream {
  quote! {
  fn exist_by_id<F>(id: Self::Id) -> sqlx::query::Map<'static, sqlx::Postgres, F, PgArguments>
  where
      F: FnMut(PgRow) -> Result<model::record::ExistRecord, sqlx::Error>{
          todo!()
      }
  }
}

fn save(_table: &Table) -> TokenStream {
  quote! {
  fn save(&'a self) -> sqlx::query::Query<'a, sqlx::Postgres, PgArguments>{
          todo!();
      }
  }
}

fn update(_table: &Table) -> TokenStream {
  quote! {
  fn update(&'a self) -> sqlx::query::Query<'a, sqlx::Postgres, PgArguments>{
          todo!();
      }
  }
}

fn delete_by_id(_table: &Table) -> TokenStream {
  quote! {
  fn delete_by_id(id: Self::Id) -> sqlx::query::Query<'a, sqlx::Postgres, PgArguments>{
          todo!()
      }
  }
}
