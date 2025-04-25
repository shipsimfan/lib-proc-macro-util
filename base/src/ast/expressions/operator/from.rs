use crate::ast::expressions::{BorrowExpression, OperatorExpression};

impl<'a> From<BorrowExpression<'a>> for OperatorExpression<'a> {
    fn from(literal: BorrowExpression<'a>) -> Self {
        OperatorExpression::Borrow(literal)
    }
}
