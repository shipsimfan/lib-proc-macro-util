use crate::ast::{expressions::LiteralExpression, Expression};

impl<'a, T: Into<LiteralExpression<'a>>> From<T> for Expression<'a> {
    fn from(literal: T) -> Self {
        Expression::Literal(literal.into())
    }
}
