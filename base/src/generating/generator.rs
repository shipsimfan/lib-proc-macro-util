use crate::{tokens::TokenTree, ToTokens};

/// An object for generating tokens
pub struct Generator<'a> {
    tokens: &'a mut Vec<TokenTree>,
}

impl<'a> Generator<'a> {
    /// Creates a new [`Generator`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Generator`]
    pub(crate) fn new(tokens: &'a mut Vec<TokenTree>) -> Self {
        Generator { tokens }
    }

    /// Generates the tokens for the requested value
    ///
    /// ## Parameters
    ///  * `value` - The value to generate tokens for
    pub fn generate<T: ToTokens + ?Sized>(&mut self, value: &T) {
        value.to_tokens(self)
    }

    /// Appends the token tree into the stream
    ///
    /// ## Parameters
    ///  * `token` - The token tree to append
    pub fn push(&mut self, token: &TokenTree) {
        self.tokens.push(token.clone().into())
    }
}
