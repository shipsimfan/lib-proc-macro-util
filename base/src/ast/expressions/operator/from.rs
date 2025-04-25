use crate::ast::expressions::{BorrowExpression, DereferenceExpression, OperatorExpression};

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
