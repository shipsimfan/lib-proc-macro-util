use crate::ast::{
    expressions::{BlockExpression, UnsafeBlockExpression},
    ExpressionWithBlockKind,
};

impl<'a> From<BlockExpression<'a>> for ExpressionWithBlockKind<'a> {
    fn from(block: BlockExpression<'a>) -> Self {
        ExpressionWithBlockKind::Block(block)
    }
}

impl<'a> From<UnsafeBlockExpression<'a>> for ExpressionWithBlockKind<'a> {
    fn from(r#unsafe: UnsafeBlockExpression<'a>) -> Self {
        ExpressionWithBlockKind::Unsafe(r#unsafe)
    }
}
