use crate::ast::expressions::{ArrayElements, ArrayExpression};

impl<'a> ArrayExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ArrayExpression<'static> {
        ArrayExpression {
            elements: self.elements.map(ArrayElements::into_static),
        }
    }
}
