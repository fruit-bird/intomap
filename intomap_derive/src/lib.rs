mod expand;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

/// A substandard way of serialization
/// 
/// # Example
/// ```
/// # use intomap_derive::IntoMap;
/// #[derive(IntoMap)]
/// struct User {
///     name: &'static str,
///     #[intomap(ignore)]
///     id: usize,
///     #[intomap(rename = "online")]
///     active: bool,
/// }
/// 
/// let user = User {
///     name: "Jimothy",
///     id: 0,
///     active: true,
/// };
/// 
/// let user_map = user.as_map();
/// let should_match = BTreeMap::from([
///     ("name".to_string(), "Jimothy".to_string()),
///     ("online".to_string(), "true".to_string()),
/// ]);
/// 
/// assert_eq!(user_map, should_match);
/// ```
#[proc_macro_derive(IntoMap, attributes(intomap))]
pub fn intomap_derive(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    expand::intomap_impl(parsed_input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
