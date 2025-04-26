use crate::ast::items::FunctionBody;

impl<'a> FunctionBody<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> FunctionBody<'static> {
        match self {
            FunctionBody::Expression(expression) => {
                FunctionBody::Expression(expression.into_static())
            }
            FunctionBody::Semi(semi) => FunctionBody::Semi(semi),
        }
    }
}
