use quote::ToTokens;

#[derive(Clone)]
pub struct ToTokenList<T: ToTokens> {
    items: Vec<T>
}

impl<T: ToTokens> FromIterator<T> for ToTokenList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self { items: Vec::from_iter(iter) }
    }
}

impl<T: ToTokens> ToTokens for ToTokenList<T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for item in &self.items {
            item.to_tokens(tokens);
        }
    }
}