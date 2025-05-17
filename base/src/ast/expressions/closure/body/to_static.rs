use crate::ast::expressions::ClosureBody;

impl<'a> ClosureBody<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ClosureBody<'static> {
        match self {
            ClosureBody::Expression(expression) => {
                ClosureBody::Expression(Box::new(expression.into_static()))
            }
            ClosureBody::ReturnType {
                arrow,
                r#type,
                expression,
            } => ClosureBody::ReturnType {
                arrow,
                r#type: Box::new(r#type.into_static()),
                expression: expression.into_static(),
            },
        }
    }
}
