extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use quote::quote;

#[proc_macro_derive(HTMLElement)]
pub fn html_element(input: TokenStream) -> TokenStream { 
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let named_fields = get_named_fields(input.data);
    
    let expanded = quote! {
        impl html_element::HTMLElement for #name {
            pub fn render(&self) -> String {

            }
        }
    };

    TokenStream::from(expanded)
}

fn get_named_fields(data: Data) -> Vec<&str> {
    match data {
        Data::Struct(ref data) => {
// https://docs.rs/syn/latest/syn/enum.Fields.html#method.members
// https://crates.io/crates/quote
// https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize_derive/src/lib.rs
        },
        _ => vec![]
    }
}

