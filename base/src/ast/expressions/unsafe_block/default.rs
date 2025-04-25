use crate::ast::expressions::UnsafeBlockExpression;

impl<'a> Default for UnsafeBlockExpression<'a> {
    fn default() -> Self {
        UnsafeBlockExpression::new()
    }
}
