use crate::ast::expressions::GroupedExpression;

impl<'a> GroupedExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> GroupedExpression<'static> {
        GroupedExpression {
            expression: Box::new(self.expression.into_static()),
        }
    }
}
