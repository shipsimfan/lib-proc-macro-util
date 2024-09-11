use crate::tokens::TokenTree;

/// Converts `tokens` into a [`proc_macro::TokenStream`]
pub fn into_token_stream(tokens: Vec<TokenTree>) -> proc_macro::TokenStream {
    let mut token_stream = proc_macro::TokenStream::new();
    token_stream.extend(
        tokens
            .into_iter()
            .map(|token| Into::<proc_macro::TokenTree>::into(token)),
    );
    token_stream
}
