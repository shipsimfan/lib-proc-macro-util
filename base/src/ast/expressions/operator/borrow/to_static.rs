use crate::ast::expressions::BorrowExpression;

impl<'a> BorrowExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> BorrowExpression<'static> {
        BorrowExpression {
            ampersand: self.ampersand,
            mutable: self.mutable,
            expression: Box::new(self.expression.into_static()),
        }
    }
}
