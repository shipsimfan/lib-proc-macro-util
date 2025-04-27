use crate::ast::{
    expressions::{BlockExpression, ConstBlockExpression, LoopExpression, UnsafeBlockExpression},
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

impl<'a> From<ConstBlockExpression<'a>> for ExpressionWithBlockKind<'a> {
    fn from(r#const: ConstBlockExpression<'a>) -> Self {
        ExpressionWithBlockKind::Const(r#const)
    }
}

impl<'a> From<LoopExpression<'a>> for ExpressionWithBlockKind<'a> {
    fn from(r#loop: LoopExpression<'a>) -> Self {
        ExpressionWithBlockKind::Loop(r#loop)
    }
}
