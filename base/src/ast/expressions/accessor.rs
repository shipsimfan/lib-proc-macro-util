use crate::{ast::Expression, tokens::Identifier, Generator, Parser, Result, ToTokens, Token};

/// An expression accessing a member of an item
///
/// Example: `foo.bar`
#[derive(Clone)]
pub struct AccessorExpression<'a> {
    /// The expression being accessed
    pub expression: Box<Expression<'a>>,

    /// The dot token inidicating access
    pub dot: Token![.],

    /// The name of thing being accessed
    pub identifier: Identifier,
}

impl<'a> AccessorExpression<'a> {
    /// Parses the [`AccessorExpression`] following `expression`
    pub fn parse(expression: Box<Expression<'a>>, parser: &mut Parser<'a>) -> Result<Self> {
        let dot = parser.parse()?;
        let identifier = parser.parse()?;

        Ok(AccessorExpression {
            expression,
            dot,
            identifier,
        })
    }
}

impl<'a> ToTokens for AccessorExpression<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        self.expression.to_tokens(generator);
        self.dot.to_tokens(generator);
        self.identifier.to_tokens(generator);
    }
}
