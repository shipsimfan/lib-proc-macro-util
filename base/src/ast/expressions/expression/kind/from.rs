use crate::ast::{
    expressions::{BlockExpression, LiteralExpression},
    ExpressionKind,
};

impl<'a> From<LiteralExpression<'a>> for ExpressionKind<'a> {
    fn from(literal: LiteralExpression<'a>) -> Self {
        ExpressionKind::Literal(literal)
    }
}

impl<'a> From<BlockExpression<'a>> for ExpressionKind<'a> {
    fn from(block: BlockExpression<'a>) -> Self {
        ExpressionKind::Block(block)
    }
}
