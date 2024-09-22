use crate::ast::expressions::BlockExpression;

impl<'a> Default for BlockExpression<'a> {
    fn default() -> Self {
        BlockExpression::new()
    }
}
