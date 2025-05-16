use crate::ast::expressions::AwaitExpression;

impl<'a> AwaitExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> AwaitExpression<'static> {
        AwaitExpression {
            expression: Box::new(self.expression.into_static()),
            dot: self.dot,
            r#await: self.r#await,
        }
    }
}
