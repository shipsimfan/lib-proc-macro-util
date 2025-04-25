use crate::ast::expressions::OperatorExpression;

impl<'a> OperatorExpression<'a> {
    /// Creates a new [`OperatorExpression`]
    pub fn new<T: Into<OperatorExpression<'a>>>(expression: T) -> Self {
        expression.into()
    }
}
