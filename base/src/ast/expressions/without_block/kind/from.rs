use crate::ast::{expressions::LiteralExpression, ExpressionWithoutBlockKind};

impl<'a> From<LiteralExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(literal: LiteralExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::Literal(literal)
    }
}
