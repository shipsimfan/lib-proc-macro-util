use crate::{
    ast::{Expression, FunctionCallExpression},
    tokens::Dot,
    Parser, Result, ToTokens,
};

/// An expression calling a function on an item
///
/// Example: `foo.bar()`
#[derive(Clone)]
pub struct FunctionAccessorExpression<'a> {
    expression: Box<Expression<'a>>,
    dot: Dot,
    function_call: FunctionCallExpression<'a>,
}

impl<'a> FunctionAccessorExpression<'a> {
    /// Parses the [`FunctionAccessorExpression`] following `expression`
    pub fn parse(expression: Box<Expression<'a>>, parser: &mut Parser<'a>) -> Result<Self> {
        let dot = parser.parse()?;
        let function_call = parser.parse()?;

        Ok(FunctionAccessorExpression {
            expression,
            dot,
            function_call,
        })
    }
}

impl<'a> ToTokens for FunctionAccessorExpression<'a> {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        self.expression.to_tokens(generator);
        self.dot.to_tokens(generator);
        self.function_call.to_tokens(generator);
    }
}
