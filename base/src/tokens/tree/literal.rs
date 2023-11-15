use crate::{Error, Generator, Parse, Parser, Result, ToTokens};
use proc_macro::Span;

/// A string or numeric literal
#[derive(Clone)]
pub struct Literal(proc_macro::Literal);

impl Literal {
    /// Creates a new [`Literal`] from a string
    ///
    /// ## Parameters
    ///  * `string` - The string for the new literal
    ///  * `span` - The span used by the new literal
    ///
    /// ## Return Value
    /// Returns the newly created [`Literal`]
    pub fn new_string(string: &str, span: Span) -> Self {
        let mut literal = proc_macro::Literal::string(string);
        literal.set_span(span);
        literal.into()
    }

    /// Get the [`Span`] of this literal
    ///
    /// ## Return Value
    /// Returns this literal's [`Span`]
    pub fn span(&self) -> Span {
        self.0.span()
    }
}

impl From<proc_macro::Literal> for Literal {
    fn from(literal: proc_macro::Literal) -> Self {
        Literal(literal)
    }
}

impl Into<proc_macro::Literal> for Literal {
    fn into(self) -> proc_macro::Literal {
        self.0
    }
}

impl Parse for Literal {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.literal().ok_or(Error::new("expected an identifier"))
    }
}

impl ToTokens for Literal {
    fn to_tokens(&self, generator: &mut Generator) {
        generator.literal(self.clone())
    }
}