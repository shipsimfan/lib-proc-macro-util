use crate::ast::expressions::{
    ArithmeticOrLogicalExpression, BorrowExpression, ComparisonExpression, DereferenceExpression,
    ErrorPropagationExpression, LazyBooleanExpression, NegationExpression, OperatorExpression,
    TypeCastExpression,
};

use super::CompoundAssignmentExpression;

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

impl<'a> From<ArithmeticOrLogicalExpression<'a>> for OperatorExpression<'a> {
    fn from(arithmetic_or_logical: ArithmeticOrLogicalExpression<'a>) -> Self {
        OperatorExpression::ArithmeticOrLogical(arithmetic_or_logical)
    }
}

impl<'a> From<LazyBooleanExpression<'a>> for OperatorExpression<'a> {
    fn from(lazy_boolean: LazyBooleanExpression<'a>) -> Self {
        OperatorExpression::LazyBoolean(lazy_boolean)
    }
}

impl<'a> From<CompoundAssignmentExpression<'a>> for OperatorExpression<'a> {
    fn from(compound_assignment: CompoundAssignmentExpression<'a>) -> Self {
        OperatorExpression::CompoundAssignment(compound_assignment)
    }
}
