use crate::{
    tokens::{OwnedTokenTree, TokenTree},
    ToTokens,
};

/// A buffer for a stream of tokens to parse
pub struct TokenBuffer {
    tokens: Vec<OwnedTokenTree>,
}

impl TokenBuffer {
    /// Creates a new [`TokenBuffer`]
    pub fn new() -> Self {
        TokenBuffer { tokens: Vec::new() }
    }

    /// Get the number of tokens in the buffer
    ///
    /// ## Return Value
    /// Returns the number of tokens in the buffer
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Gets the [`TokenTree`] located at `index`
    pub fn get(&self, index: usize) -> Option<TokenTree> {
        self.tokens.get(index).map(|token| token.into())
    }

    /// Pushes a new [`OwnedTokenTree`] to the end of the buffer
    pub(crate) fn push(&mut self, token: OwnedTokenTree) {
        self.tokens.push(token)
    }

    /// Returns the last element in the buffer, if it exists
    pub(crate) fn last_mut(&mut self) -> Option<&mut OwnedTokenTree> {
        self.tokens.last_mut()
    }
}

impl From<proc_macro::TokenStream> for TokenBuffer {
    fn from(token_stream: proc_macro::TokenStream) -> Self {
        let mut tokens = Vec::new();
        for token in token_stream.into_iter() {
            tokens.push(token.into())
        }
        TokenBuffer { tokens }
    }
}

impl From<Vec<OwnedTokenTree>> for TokenBuffer {
    fn from(tokens: Vec<OwnedTokenTree>) -> Self {
        TokenBuffer { tokens }
    }
}

impl Into<proc_macro::TokenStream> for TokenBuffer {
    fn into(self) -> proc_macro::TokenStream {
        self.tokens
            .into_iter()
            .map(|token| -> proc_macro::TokenTree { token.into() })
            .collect()
    }
}

impl ToTokens for TokenBuffer {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        for token in &self.tokens {
            generator.push(&token.into())
        }
    }
}
