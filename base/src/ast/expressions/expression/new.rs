use crate::ast::Expression;

impl<'a> Expression<'a> {
    /// Creates a new [`Expression`]
    pub fn new<T: Into<Expression<'a>>>(expression: T) -> Self {
        expression.into()
    }
}
