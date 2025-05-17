use crate::ast::expressions::CompoundAssignmentExpression;

impl<'a> CompoundAssignmentExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> CompoundAssignmentExpression<'static> {
        CompoundAssignmentExpression {
            left: Box::new(self.left.into_static()),
            operator: self.operator,
            right: Box::new(self.right.into_static()),
        }
    }
}
