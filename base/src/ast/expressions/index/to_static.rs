use crate::ast::expressions::IndexExpression;

impl<'a> IndexExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> IndexExpression<'static> {
        IndexExpression {
            expression: Box::new(self.expression.into_static()),
            index: Box::new(self.index.into_static()),
        }
    }
}
