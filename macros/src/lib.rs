use syn::{__private::TokenStream, parse};
use quote::quote;

mod tensor_literal;

#[proc_macro]
pub fn tensor(input: TokenStream) -> TokenStream {
    let literal = parse::<tensor_literal::TensorLiteral>(input).unwrap();
    TokenStream::from(quote!(#literal))
}