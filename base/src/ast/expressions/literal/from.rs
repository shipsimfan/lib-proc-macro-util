use crate::{ast::expressions::LiteralExpression, tokens::Literal, Token};

impl<'a> From<&'a Literal> for LiteralExpression<'a> {
    fn from(literal: &'a Literal) -> Self {
        LiteralExpression::Literal(literal.into())
    }
}

impl<'a, T: Into<Literal>> From<T> for LiteralExpression<'a> {
    fn from(literal: T) -> Self {
        LiteralExpression::new(literal.into())
    }
}

impl<'a> From<Token![true]> for LiteralExpression<'a> {
    fn from(r#true: Token![true]) -> Self {
        LiteralExpression::True(r#true)
    }
}

impl<'a> From<Token![false]> for LiteralExpression<'a> {
    fn from(r#false: Token![false]) -> Self {
        LiteralExpression::False(r#false)
    }
}

impl<'a> From<bool> for LiteralExpression<'a> {
    fn from(value: bool) -> Self {
        LiteralExpression::new_bool(value)
    }
}
