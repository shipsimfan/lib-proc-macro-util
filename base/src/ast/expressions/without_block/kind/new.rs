use crate::ast::ExpressionWithoutBlockKind;

impl<'a> ExpressionWithoutBlockKind<'a> {
    /// Creates a new [`ExpressionWithoutBlockKind`]
    pub fn new<T: Into<ExpressionWithoutBlockKind<'a>>>(expression: T) -> Self {
        expression.into()
    }
}
