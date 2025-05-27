use std::collections::HashMap;

use syn::{Error, spanned::Spanned};

mod enum_constant_parsing;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(html))]
struct HTMLStructAttributes {
    tag_name: String,
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(html))]
struct HTMLFieldAttributes {
    value: String,
}

fn html_derive_macro2(item: proc_macro2::TokenStream) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: syn::DeriveInput = syn::parse2(item)?;
    let HTMLStructAttributes { tag_name } = deluxe::extract_attributes(&mut ast)?;
    let mut binding = ast.clone();
    let field_attrs: HashMap<String, HTMLFieldAttributes> = extract_html_field_attrs(&mut binding)?;

    let ident = ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    let opening_tag = create_opening_tag(&tag_name, field_attrs);

    Ok(quote::quote! {
        impl #impl_generics HTMLElement for #ident #type_generics #where_clause {
            fn render() -> String {
                format!(
                    "{}\n{}\n</{}>",
                    #opening_tag,
                    "",
                    #tag_name,
                )
            }
        }
    })
}

fn create_opening_tag(tag_name: &str, fields: HashMap<String, HTMLFieldAttributes>) -> String {
    if fields.is_empty() {
        format!("<{}>", tag_name)
    } else {
        format!("<{} {}>", tag_name, build_properties(fields))
    }
}

/// Surround an expression in quotes, useful for building HTML elements
///
/// quotes!("hi") --> ""hi""
/// quotes!(9) --> "9"
macro_rules! quotes {
    ($value:expr) => {
        format!(r#""{}""#, $value)
    };
}

/// Create HTML properties for a specfic tag
///
/// These properties take the form of
/// `key = "value" second_key = "new_value"`
///
/// See [tag] for a macro to create a full HTML tag, with a body and a closing tag
macro_rules! properties {
    ($key:expr, $value:expr) => {
        format!("{}={}", $key, quotes!($value))
    };
    ($($key:expr, $value:expr);+) => {
        {
            let mut v: Vec<String> = Vec::new();

            $(
                v.push(properties!($key, $value));
            )+

           v.join(" ")
        }
    };
}

fn build_properties(fields: HashMap<String, HTMLFieldAttributes>) -> String {
    let mut properties: Vec<String> = Vec::new();

    for (field_name, attributes) in fields.iter() {
        properties.push(properties!(field_name, attributes.value))
    }

    properties.join(" ")
}

fn extract_html_field_attrs(
    input: &mut syn::DeriveInput,
) -> deluxe::Result<HashMap<String, HTMLFieldAttributes>> {
    let mut field_attrs: HashMap<String, HTMLFieldAttributes> = HashMap::new();

    match &mut input.data {
        syn::Data::Enum(s) => {
            for variant in s.variants.iter_mut() {
                let attrs: HTMLFieldAttributes = deluxe::extract_attributes(variant)?;

                field_attrs.insert(variant.ident.to_string(), attrs);
            }

            Ok(field_attrs)
        }
        syn::Data::Struct(d) => Err(Error::new(
            d.struct_token.span(),
            format!(
                "This macro is only supported on Enum Types, got {}",
                "Struct"
            ),
        )),
        syn::Data::Union(d) => Err(Error::new(
            d.union_token.span(),
            format!(
                "This macro is only supported on Enum Types, got {}",
                "Union"
            ),
        )),
    }
}

/*
pub trait HTMLElement {
    fn render(&self) -> String;
}
 */

#[proc_macro_derive(HTMLElement, attributes(html))]
pub fn html_element_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    html_derive_macro2(input.into()).unwrap().into()
}

#[cfg(test)]
mod properties_tests {
    #[test]
    fn should_build_single_property() {
        let key = "key";
        let value = "value";

        let expected = format!("{}=\"{}\"", key, value);

        let actual = properties!(key, value);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_build_multiple_properties() {
        let key_one = "key_one";
        let value_one = "value_one";
        let key_two = "key_two";
        let value_two = "value_two";

        let expected = format!(
            "{}=\"{}\" {}=\"{}\"",
            key_one, value_one, key_two, value_two
        );

        let actual = properties!(key_one, value_one; key_two, value_two);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_build_property_of_i32() {
        let key = "key";
        let value: i32 = 4;

        let expected = format!("{}=\"{}\"", key, value);

        let actual = properties!(key, value);

        assert_eq!(actual, expected);
    }
}

