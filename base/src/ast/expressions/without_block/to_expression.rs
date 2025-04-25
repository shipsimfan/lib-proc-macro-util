use crate::ast::{Expression, ExpressionWithoutBlock};

impl<'a> ExpressionWithoutBlock<'a> {
    /// Convert this into a full [`Expression`]
    pub fn into_expression(self) -> Expression<'a> {
        Expression {
            attributes: self.attributes,
            kind: self.kind.into_expression(),
        }
    }
}
