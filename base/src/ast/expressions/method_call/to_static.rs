use crate::ast::expressions::MethodCallExpression;

impl<'a> MethodCallExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> MethodCallExpression<'static> {
        MethodCallExpression {
            function: Box::new(self.function.into_static()),
            dot: self.dot,
            name: self.name.into_static(),
            parameters: self.parameters.into_static(),
        }
    }
}
