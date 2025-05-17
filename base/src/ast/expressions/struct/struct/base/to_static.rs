use crate::ast::expressions::StructBase;

impl<'a> StructBase<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructBase<'static> {
        StructBase {
            dots: self.dots,
            expression: Box::new(self.expression.into_static()),
        }
    }
}
