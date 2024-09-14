use crate::tokens::TokenTree;

/// Collects the tokens from a [`proc_macro::TokenStream`] into a list usable by this crate
pub fn collect_token_stream(stream: proc_macro::TokenStream) -> Vec<TokenTree> {
    let mut tokens = Vec::new();
    tokens.extend(stream.into_iter().map(|token| token.into()));
    tokens
}
