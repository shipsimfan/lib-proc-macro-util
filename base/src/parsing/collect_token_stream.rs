use crate::tokens::TokenTree;
use proc_macro::TokenStream;

/// Collects the tokens from a [`TokenStream`] into a list usable by this crate
pub fn collect_token_stream(stream: TokenStream) -> Vec<TokenTree> {
    let mut tokens = Vec::new();
    tokens.extend(stream.into_iter().map(|token| token.into()));
    tokens
}
