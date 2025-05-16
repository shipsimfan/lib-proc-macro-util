use crate::ast::expressions::OperatorExpression;

impl<'a> OperatorExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> OperatorExpression<'static> {
        match self {
            OperatorExpression::Borrow(borrow) => OperatorExpression::Borrow(borrow.into_static()),
            OperatorExpression::Dereference(dereference) => {
                OperatorExpression::Dereference(dereference.into_static())
            }
            OperatorExpression::Comparison(comparison) => {
                OperatorExpression::Comparison(comparison.into_static())
            }
            OperatorExpression::ErrorPropagation(error_propagation) => {
                OperatorExpression::ErrorPropagation(error_propagation.into_static())
            }
        }
    }
}
