use crate::ast::expressions::ReturnExpression;

impl<'a> ReturnExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ReturnExpression<'static> {
        ReturnExpression {
            r#return: self.r#return,
            expression: self
                .expression
                .map(|expression| Box::new(expression.into_static())),
        }
    }
}
