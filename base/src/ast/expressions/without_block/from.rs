use crate::ast::{expressions::LiteralExpression, ExpressionWithoutBlock};

impl<'a, T: Into<LiteralExpression<'a>>> From<T> for ExpressionWithoutBlock<'a> {
    fn from(literal: T) -> Self {
        ExpressionWithoutBlock::Literal(literal.into())
    }
}
