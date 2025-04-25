use crate::ast::expressions::OperatorExpression;

impl<'a> OperatorExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> OperatorExpression<'static> {
        match self {
            OperatorExpression::Borrow(borrow) => OperatorExpression::Borrow(borrow.into_static()),
        }
    }
}
