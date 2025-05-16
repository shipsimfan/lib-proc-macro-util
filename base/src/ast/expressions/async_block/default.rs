use crate::ast::expressions::AsyncBlockExpression;

impl<'a> Default for AsyncBlockExpression<'a> {
    fn default() -> Self {
        AsyncBlockExpression::new()
    }
}
