use crate::ast::expressions::AssignmentExpression;

impl<'a> AssignmentExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> AssignmentExpression<'static> {
        AssignmentExpression {
            left: Box::new(self.left.into_static()),
            equals: self.equals,
            right: Box::new(self.right.into_static()),
        }
    }
}
