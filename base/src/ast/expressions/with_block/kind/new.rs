use crate::ast::ExpressionWithBlockKind;

impl<'a> ExpressionWithBlockKind<'a> {
    /// Creates a new [`ExpressionWithBlockKind`]
    pub fn new<T: Into<ExpressionWithBlockKind<'a>>>(expression: T) -> Self {
        expression.into()
    }
}
