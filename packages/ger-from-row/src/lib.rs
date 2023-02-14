//! ```
//! #[derive(ToSql, FromSql)]
//! enum UserRole {
//!     Admin,
//!     User,
//! }
//!
//! #[derive(FromRow)]
//! struct User {
//!     #[fromrow(field = "userUsername")]
//!     user_username: String,
//!     user_email: String,
//!     user_role: UserRole,
//! }
//! ```

use syn::ext::IdentExt;

#[proc_macro_derive(FromRow, attributes(fromrow))]
pub fn from_row(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    // match struct or enum implementation
    match ast.data {
        syn::Data::Struct(ref s) => {
            return implement_struct_try_from(&ast, s);
        }
        _ => panic!("other than struct cannot be mapped"),
    };
}

struct MacroAttr(String);

/// Get field attribute from the struct/enum attributes. The function returns `None` for all failed
/// cases and the actual value from the attribute in a form of a custom struct so the data can be
/// used to further generate correct macro output.
fn parse_field_attribute(attrs: &Vec<syn::Attribute>) -> Option<MacroAttr> {
    for attr in attrs {
        match attr.parse_meta() {
            Ok(syn::Meta::List(ref ml)) => {
                for seg in &ml.path.segments {
                    if seg.ident == "fromrow" {
                        for w in &ml.nested {
                            match w {
                                syn::NestedMeta::Meta(ref me) => match me {
                                    syn::Meta::NameValue(ref nv) => {
                                        if nv.path.is_ident("field") {
                                            match nv.lit {
                                                syn::Lit::Str(ref lit) => {
                                                    return Some(MacroAttr(lit.value()));
                                                }
                                                _ => return None,
                                            }
                                        }
                                    }
                                    _ => {
                                        return None;
                                    }
                                },
                                _ => return None,
                            }
                        }
                    }
                }
                return None;
            }
            _what => {
                return None;
            }
        };
    }

    None
}

fn implement_struct_try_from(
    ast: &syn::DeriveInput,
    struct_data: &syn::DataStruct,
) -> proc_macro::TokenStream {
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let name = &ast.ident;

    let struct_fields = struct_data.fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        let row_expr = format!(r##"{}"##, ident.unraw());

        // field with attributes
        if field.attrs.len() > 0 {
            if let Some(new_name) = parse_field_attribute(&field.attrs) {
                let type_name = new_name.0;

                return quote::quote! {
                    #ident: row.try_get::<&str, #ty>(#type_name)?
                };
            }
        }

        quote::quote! {
            #ident: row.try_get::<&str, #ty>(#row_expr)?
        }
    });

    let another_struct_fields = struct_fields.clone();

    let tokens = quote::quote! {
        impl #impl_generics ::std::convert::TryFrom<&tokio_postgres::row::Row> for #name #ty_generics #where_clause {
            type Error = anyhow::Error;

            fn try_from(row: &tokio_postgres::row::Row) -> ::std::result::Result<Self, Self::Error> {
                Ok(Self {
                    #(#struct_fields),*
                })
            }
        }

        impl #impl_generics ::std::convert::TryFrom<tokio_postgres::row::Row> for #name #ty_generics #where_clause {
            type Error = anyhow::Error;

            fn try_from(row: tokio_postgres::row::Row) -> ::std::result::Result<Self, Self::Error> {
                Ok(Self {
                    #(#another_struct_fields),*
                })
            }
        }
    };

    tokens.into()
}
