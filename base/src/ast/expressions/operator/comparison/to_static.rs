use crate::ast::expressions::ComparisonExpression;

impl<'a> ComparisonExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ComparisonExpression<'static> {
        ComparisonExpression {
            left: Box::new(self.left.into_static()),
            operator: self.operator,
            right: Box::new(self.right.into_static()),
        }
    }
}
