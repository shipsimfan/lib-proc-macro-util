use crate::ast::expressions::CallExpression;

impl<'a> CallExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> CallExpression<'static> {
        CallExpression {
            function: Box::new(self.function.into_static()),
            parameters: self.parameters.into_static(),
        }
    }
}
