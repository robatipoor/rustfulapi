mod query;
mod table;

#[proc_macro_derive(BaseQuery, attributes(base_query))]
pub fn derive_base_query(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  match table::derive(input) {
    Ok(ok) => ok,
    Err(err) => err.to_compile_error(),
  }
  .into()
}
