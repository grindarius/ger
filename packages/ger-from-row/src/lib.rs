//! ```
//! #[derive(FromRow)]
//! enum UserRole {
//!     #[fromrow(field = "admin")]
//!     Admin,
//!     #[fromrow(field = "user")]
//!     User,
//! }
//!
//! #[derive(FromRow)]
//! struct User {
//!     #[fromrow(field = "userUsername")]
//!     user_username: String,
//!     user_email: String,
//!     #[fromrow(enum = "user_role")]
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
        syn::Data::Enum(ref e) => {
            return implement_enum_try_from(&ast, e);
        }
        _ => panic!("other than struct and enum cannot be mapped"),
    };
}

enum MacroAttribute {
    Enum(Option<String>),
    Field(String),
}

/// Get field attribute from the struct/enum attributes. The function returns `None` for all failed
/// cases and the actual value from the attribute in a form of a custom struct so the data can be
/// used to further generate correct macro output.
fn parse_field_attribute(attrs: &Vec<syn::Attribute>) -> Option<MacroAttribute> {
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
                                                    return Some(MacroAttribute::Field(
                                                        lit.value(),
                                                    ));
                                                }
                                                _ => return None,
                                            }
                                        }

                                        if nv.path.is_ident("num") {
                                            match nv.lit {
                                                syn::Lit::Str(ref lit) => {
                                                    return Some(MacroAttribute::Enum(Some(
                                                        lit.value(),
                                                    )))
                                                }
                                                _ => return None,
                                            }
                                        }
                                    }
                                    syn::Meta::Path(ref p) => {
                                        if let Some(f) = p.segments.first() {
                                            if f.ident.to_string() == "num" {
                                                return Some(MacroAttribute::Enum(None));
                                            }
                                        }

                                        return None;
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
        let ident_string = field.ident.as_ref().unwrap().to_string();

        // field with attributes
        if field.attrs.len() > 0 {
            if let Some(new_name) = parse_field_attribute(&field.attrs) {
                match new_name {
                    // #[fromrow(field = "some_name")]
                    crate::MacroAttribute::Field(f) => {
                        return quote::quote! {
                            #ident: row.try_get::<&str, #ty>(#f)?
                        };
                    }
                    crate::MacroAttribute::Enum(enum_rename) => {
                        // #[fromrow(num = "some_name")]
                        if let Some(rename) = enum_rename {
                            return quote::quote! {
                                #ident: #ty::try_from(row.try_get::<&str, ::std::string::String>(#rename)?)?
                            };
                        } else {
                            // #[fromrow(num)]
                            return quote::quote! {
                                #ident: #ty::try_from(row.try_get::<&str, ::std::string::String>(#ident_string)?)?
                            }
                        }
                    }
                }
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

fn implement_enum_try_from(
    ast: &syn::DeriveInput,
    enum_data: &syn::DataEnum,
) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    let enum_fields = enum_data.variants.iter().map(|variant| {
        let normalized_variant_name = &variant.ident.to_string().to_lowercase();
        let variant_name = &variant.ident;

        if variant.attrs.len() > 0 {
            if let Some(rename) = parse_field_attribute(&variant.attrs) {
                match rename {
                    MacroAttribute::Enum(e) => {
                        return quote::quote! {
                            #e => {
                                return Ok(Self::#variant_name);
                            }
                        }
                    }
                    MacroAttribute::Field(f) => {
                        return quote::quote! {
                            #f => {
                                return Ok(Self::#variant_name);
                            }
                        }
                    }
                };
            }
        }

        quote::quote! {
            #normalized_variant_name => {
                return Ok(Self::#variant_name);
            }
        }
    });

    let another_enum_fields = enum_fields.clone();

    // ty_generics and where_clause might have to go on other place that there. currently unknown
    let tokens = quote::quote! {
        impl #impl_generics ::std::convert::TryFrom<::std::string::String> for #name #ty_generics #where_clause {
            type Error = anyhow::Error;

            fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, Self::Error> {
                match value.as_str() {
                    #(#enum_fields)*
                    what => Err(anyhow::anyhow!("cannot parse match arm {}", what)),
                }
            }
        }

        impl #impl_generics ::std::convert::TryFrom<&::std::primitive::str> for #name #ty_generics #where_clause {
            type Error = anyhow::Error;

            fn try_from(value: &::std::primitive::str) -> ::std::result::Result<Self, Self::Error> {
                match value {
                    #(#another_enum_fields)*
                    what => Err(anyhow::anyhow!("cannot parse match arm {}", what)),
                }
            }
        }
    };

    tokens.into()
}
