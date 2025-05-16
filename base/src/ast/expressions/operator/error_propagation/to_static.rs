use crate::ast::expressions::ErrorPropagationExpression;

impl<'a> ErrorPropagationExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ErrorPropagationExpression<'static> {
        ErrorPropagationExpression {
            expression: Box::new(self.expression.into_static()),
            question: self.question,
        }
    }
}
