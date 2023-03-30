use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Error, Result, Type, Visibility};

pub struct Table {
  pub ident: Ident,
  pub vis: Visibility,
  pub table: String,
  pub id: TableField,
  pub fields: Vec<TableField>,
}

#[derive(Clone)]
pub struct TableField {
  pub field: Ident,
  pub ty: Type,
  pub column_name: String,
  pub custom_type: bool,
}

impl TryFrom<&syn::DeriveInput> for Table {
  type Error = Error;

  fn try_from(value: &DeriveInput) -> Result<Self> {
    let _data = match &value.data {
      Data::Struct(s) => s,
      _ => panic!("not a struct with named fields"),
    };
    todo!()
  }
}

impl TryFrom<&syn::Field> for TableField {
  type Error = Error;

  fn try_from(value: &syn::Field) -> Result<Self> {
    let _ident = value.ident.clone().unwrap();
    todo!()
  }
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
  let parsed = Table::try_from(&input)?;
  let impl_query = crate::query::impl_base_query(&parsed);
  Ok(quote! {
      #impl_query
  })
}
