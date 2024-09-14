use crate::{parsing::Parser, tokens::TokenTree, Delimiter, Generator, Span};

mod from;
mod into;
mod new;
mod parse;
mod to_tokens;

/// A delimited group of tokens
#[derive(Debug, Clone)]
pub struct Group {
    /// The span which covers this group
    pub span: Span,

    /// The delimiter dividing the group
    pub delimiter: Delimiter,

    /// The tokens contained by this buffer
    pub tokens: Vec<TokenTree>,
}

impl Group {
    /// Creates a [`Parser`] for this groups tokens
    pub fn parser(&self) -> Parser {
        Parser::new(&self.tokens)
    }

    /// Creates a generator over the tokens of this group
    pub fn generator(&mut self) -> Generator {
        Generator::new(&mut self.tokens)
    }
}
