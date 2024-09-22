use crate::ast::ExpressionKind;

impl<'a> ExpressionKind<'a> {
    /// Creates a new [`ExpressionKind`]
    pub fn new<T: Into<ExpressionKind<'a>>>(expression: T) -> Self {
        expression.into()
    }
}
