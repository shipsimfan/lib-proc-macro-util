use crate::ast::types::BareFunctionReturnType;

impl<'a> BareFunctionReturnType<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> BareFunctionReturnType<'static> {
        BareFunctionReturnType {
            right_arrow: self.right_arrow,
            r#type: Box::new(self.r#type.into_static()),
        }
    }
}
