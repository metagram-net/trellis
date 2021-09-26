extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemStruct, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    // TODO: I wish I didn't have to clone this just for error messages :\
    let args = attr.clone();
    let attr_ast = parse_macro_input!(attr as AttributeArgs);

    let route = match &attr_ast[..] {
        [NestedMeta::Lit(Lit::Str(r))] => r,
        _ => panic!("expected 1 string argument, got [{}]", args),
    };

    let item_ast = parse_macro_input!(item as ItemStruct);

    // Build the trait implementation
    impl_post(route, &item_ast)
}

fn impl_post(route: &syn::LitStr, item: &ItemStruct) -> TokenStream {
    let struct_name = &item.ident;
    let generics = &item.generics;
    let gen = quote! {
        #item
        impl #generics ::stytch::endpoint::Endpoint for #struct_name #generics {
            fn method(&self) -> ::http::Method {
                ::http::Method::POST
            }
            fn path(&self) -> ::std::string::String {
                ::std::string::String::from(#route)
            }
            fn query(&self) -> ::stytch::endpoint::Query {
                ::stytch::endpoint::Query::default()
            }
            fn body(&self) -> ::serde_json::Result<::stytch::endpoint::Body> {
                ::stytch::endpoint::Body::from_data(self)
            }
        }
    };
    gen.into()
}
