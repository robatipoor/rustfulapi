mod query;
mod table;

#[proc_macro_derive(BaseQuery, attributes(base_query))]
pub fn derive_base_query(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  match table::derive(syn::parse_macro_input!(input as syn::DeriveInput)) {
    Ok(ok) => ok,
    Err(err) => err.to_compile_error(),
  }
  .into()
}
