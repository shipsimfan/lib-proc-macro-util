use crate::{
    ast::Expression,
    tokens::{Ampersand, Mut},
    Parse, Parser, ToTokens,
};

/// A reference expression
///
/// Example: `&foo`
#[derive(Clone)]
pub struct ReferenceExpression<'a> {
    /// The ampersand indicating the reference
    pub ampersand: Ampersand,

    /// Is this a mutable reference
    pub r#mut: Option<Mut>,

    /// The expression being referenced
    pub expression: Box<Expression<'a>>,
}

impl<'a> Parse<'a> for ReferenceExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> crate::Result<Self> {
        let ampersand = parser.parse()?;
        let r#mut = parser.parse()?;
        let expression = parser.parse()?;

        Ok(ReferenceExpression {
            ampersand,
            r#mut,
            expression,
        })
    }
}

impl<'a> ToTokens for ReferenceExpression<'a> {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        self.ampersand.to_tokens(generator);
        self.r#mut.to_tokens(generator);
        self.expression.to_tokens(generator);
    }
}
