use crate::ast::{expressions::BlockExpression, ExpressionWithBlockKind};

impl<'a> From<BlockExpression<'a>> for ExpressionWithBlockKind<'a> {
    fn from(block: BlockExpression<'a>) -> Self {
        ExpressionWithBlockKind::Block(block)
    }
}
