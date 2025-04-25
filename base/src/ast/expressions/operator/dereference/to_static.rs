use crate::ast::expressions::DereferenceExpression;

impl<'a> DereferenceExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> DereferenceExpression<'static> {
        DereferenceExpression {
            asterick: self.asterick,
            expression: Box::new(self.expression.into_static()),
        }
    }
}
