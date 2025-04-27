use crate::ast::expressions::ConstBlockExpression;

impl<'a> Default for ConstBlockExpression<'a> {
    fn default() -> Self {
        ConstBlockExpression::new()
    }
}
