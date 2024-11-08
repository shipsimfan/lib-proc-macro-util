use crate::ast::expressions::LiteralExpression;

impl<'a> std::fmt::Display for LiteralExpression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralExpression::Literal(literal) => literal.fmt(f),
            LiteralExpression::True(r#true) => r#true.fmt(f),
            LiteralExpression::False(r#false) => r#false.fmt(f),
        }
    }
}
