use crate::ast::expressions::{
    BorrowExpression, ComparisonExpression, DereferenceExpression, ErrorPropagationExpression,
    NegationExpression, OperatorExpression, TypeCastExpression,
};

impl<'a> From<BorrowExpression<'a>> for OperatorExpression<'a> {
    fn from(borrow: BorrowExpression<'a>) -> Self {
        OperatorExpression::Borrow(borrow)
    }
}

impl<'a> From<DereferenceExpression<'a>> for OperatorExpression<'a> {
    fn from(dereference: DereferenceExpression<'a>) -> Self {
        OperatorExpression::Dereference(dereference)
    }
}

impl<'a> From<ComparisonExpression<'a>> for OperatorExpression<'a> {
    fn from(comparison: ComparisonExpression<'a>) -> Self {
        OperatorExpression::Comparison(comparison)
    }
}

impl<'a> From<ErrorPropagationExpression<'a>> for OperatorExpression<'a> {
    fn from(error_propagation: ErrorPropagationExpression<'a>) -> Self {
        OperatorExpression::ErrorPropagation(error_propagation)
    }
}

impl<'a> From<TypeCastExpression<'a>> for OperatorExpression<'a> {
    fn from(type_cast: TypeCastExpression<'a>) -> Self {
        OperatorExpression::TypeCast(type_cast)
    }
}

impl<'a> From<NegationExpression<'a>> for OperatorExpression<'a> {
    fn from(negation: NegationExpression<'a>) -> Self {
        OperatorExpression::Negation(negation)
    }
}
