use crate::ast::{Expression, ExpressionWithoutBlock};

impl<'a> ExpressionWithoutBlock<'a> {
    /// Convert this into a full [`Expression`]
    pub fn into_expression(self) -> Expression<'a> {
        Expression::WithoutBlock(self)
    }
}
