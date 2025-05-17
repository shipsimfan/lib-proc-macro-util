use crate::ast::expressions::LazyBooleanExpression;

impl<'a> LazyBooleanExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> LazyBooleanExpression<'static> {
        LazyBooleanExpression {
            left: Box::new(self.left.into_static()),
            operator: self.operator,
            right: Box::new(self.right.into_static()),
        }
    }
}
