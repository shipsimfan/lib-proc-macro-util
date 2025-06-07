use crate::{
    tokens::{Group, TokenTree},
    Delimiter, ToTokens,
};

/// An object for generating tokens
pub struct Generator<'a> {
    tokens: &'a mut Vec<TokenTree>,
}

impl<'a> Generator<'a> {
    /// Creates a new [`Generator`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Generator`]
    pub fn new(tokens: &'a mut Vec<TokenTree>) -> Self {
        Generator { tokens }
    }

    /// Generates the tokens for the requested value
    ///
    /// ## Parameters
    ///  * `value` - The value to generate tokens for
    pub fn generate<T: ToTokens>(&mut self, value: T) {
        value.to_tokens(self)
    }

    /// Appends the token tree into the stream
    ///
    /// ## Parameters
    ///  * `token` - The token tree to append
    pub fn push(&mut self, token: TokenTree) {
        self.tokens.push(token.clone().into())
    }

    /// Inserts a new group with delimited by `delimiter` and returns a generator for that group
    pub fn group<'b>(&'b mut self, delimiter: Delimiter) -> Generator<'b> {
        self.tokens.push(Group::new(delimiter).into());
        match self.tokens.last_mut() {
            Some(TokenTree::Group(group)) => group.generator(),
            _ => unreachable!(),
        }
    }

    /// Inserts a new group with delimited by [`Delimiter::Brace`] and returns a generator for that
    /// group
    pub fn group_brace<'b>(&'b mut self) -> Generator<'b> {
        self.group(Delimiter::Brace)
    }

    /// Inserts a new group with delimited by [`Delimiter::Bracket`] and returns a generator for
    /// that group
    pub fn group_bracket<'b>(&'b mut self) -> Generator<'b> {
        self.group(Delimiter::Bracket)
    }

    /// Inserts a new group with delimited by [`Delimiter::Parenthesis`] and returns a generator
    /// for that group
    pub fn group_parenthesis<'b>(&'b mut self) -> Generator<'b> {
        self.group(Delimiter::Parenthesis)
    }
}
