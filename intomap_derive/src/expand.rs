use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    Data, DataStruct, DeriveInput, Error, Expr, ExprAssign, ExprLit, ExprPath, Field, Fields,
    FieldsNamed, Ident, Lit, Result,
};

const ATTR_INTOMAP: &str = "intomap";
const IDENT_IGNORE: &str = "ignore";
const IDENT_RENAME: &str = "rename";

pub fn intomap_impl(parsed_input: DeriveInput) -> Result<TokenStream2> {
    let struct_name = parsed_input.ident;
    let insert_tokens = match parsed_input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => named.iter().filter_map(|field| {
            let field_name = field.ident.clone().unwrap();
            let field_rename = field_rename(field).unwrap_or(field_name.clone());

            field
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident(ATTR_INTOMAP))
                .all(|attr| match attr.parse_args::<Ident>() {
                    Ok(ident) => ident != IDENT_IGNORE,
                    Err(_) => true,
                })
                .then_some(quote! {
                    map.insert(
                        stringify!(#field_rename).to_string(),
                        self.#field_name.to_string()
                    );
                })
        }),
        _ => {
            return Err(Error::new(
                Span::call_site(),
                "#[derive(IntoMap)] is only valid for structs with named fields",
            ))
        }
    };

    let tokens = quote! {
        use std::collections::BTreeMap;
        use intomap::IntoMap;

        impl IntoMap for #struct_name {
            fn as_map(&self) -> BTreeMap<String, String> {
                let mut map = BTreeMap::new();
                #(#insert_tokens)*
                map
            }
        }
    };

    Ok(tokens)
}

/// Returns the field identifier `Ident(new_name)` if the field has a `#[intomap(rename = "new_name")]` attribute
///
/// # Examples
/// ```ignore
/// let new_names = fields.iter().map(|field| {
///     let name = field.ident.clone().unwrap();
///     let rename = field_rename(&field).unwrap_or(name);
///     quote! { #rename }
/// });
/// ```
fn field_rename(field: &Field) -> Option<Ident> {
    field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident(ATTR_INTOMAP))
        .find_map(|attr| {
            attr.parse_args::<ExprAssign>()
                .ok()
                .and_then(|e| match (*e.left, *e.right) {
                    (
                        Expr::Path(ExprPath { path, .. }),
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(s), ..
                        }),
                    ) if path.is_ident(IDENT_RENAME) => {
                        Some(Ident::new(s.value().as_str(), s.span()))
                    }
                    _ => None,
                })
        })
}
