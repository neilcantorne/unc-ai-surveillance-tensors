use syn::{__private::TokenStream, parse};
use quote::quote;

mod tensor_literal;
mod dl_bindings;
mod to_tokens_list;

#[proc_macro]
pub fn tensor(input: TokenStream) -> TokenStream {
    let literal = parse::<tensor_literal::TensorLiteral>(input).unwrap();
    TokenStream::from(quote!(#literal))
}

#[proc_macro_attribute]
pub fn bind_open_cl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let bindings = parse::<dl_bindings::DlBinding>(item).unwrap();
    bindings.generate_open_cl()
}