use crate::ast::{expressions::LiteralExpression, ExpressionKind};

impl<'a> From<LiteralExpression<'a>> for ExpressionKind<'a> {
    fn from(literal: LiteralExpression<'a>) -> Self {
        ExpressionKind::Literal(literal)
    }
}
