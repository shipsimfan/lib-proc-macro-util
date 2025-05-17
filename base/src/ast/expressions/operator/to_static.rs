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
            OperatorExpression::TypeCast(type_cast) => {
                OperatorExpression::TypeCast(type_cast.into_static())
            }
            OperatorExpression::Negation(negation) => {
                OperatorExpression::Negation(negation.into_static())
            }
            OperatorExpression::ArithmeticOrLogical(arithmetic_or_logical) => {
                OperatorExpression::ArithmeticOrLogical(arithmetic_or_logical.into_static())
            }
            OperatorExpression::LazyBoolean(lazy_boolean) => {
                OperatorExpression::LazyBoolean(lazy_boolean.into_static())
            }
        }
    }
}
