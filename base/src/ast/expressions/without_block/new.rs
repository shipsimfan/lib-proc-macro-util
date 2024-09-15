use crate::ast::ExpressionWithoutBlock;

impl<'a> ExpressionWithoutBlock<'a> {
    /// Creates a new [`ExpressionWithoutBlock`]
    pub fn new<T: Into<ExpressionWithoutBlock<'a>>>(expression: T) -> Self {
        expression.into()
    }
}
