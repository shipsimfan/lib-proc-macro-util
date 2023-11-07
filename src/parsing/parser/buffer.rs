use proc_macro::{TokenStream, TokenTree};

pub(super) struct TokenBuffer {
    tokens: Box<[TokenTree]>,
}

impl TokenBuffer {
    pub(super) fn new(input: TokenStream) -> Self {
        let tokens = input.into_iter().collect();

        TokenBuffer { tokens }
    }

    pub(super) fn as_slice(&self) -> &[TokenTree] {
        &self.tokens
    }
}
