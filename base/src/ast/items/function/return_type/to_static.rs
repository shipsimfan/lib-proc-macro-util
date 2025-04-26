use crate::ast::items::FunctionReturnType;

impl<'a> FunctionReturnType<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> FunctionReturnType<'static> {
        FunctionReturnType {
            arrow: self.arrow,
            r#type: self.r#type.into_static(),
        }
    }
}
