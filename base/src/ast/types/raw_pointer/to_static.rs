use crate::ast::types::RawPointerType;

impl<'a> RawPointerType<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> RawPointerType<'static> {
        RawPointerType {
            asterick: self.asterick,
            mutability: self.mutability,
            r#type: Box::new(self.r#type.into_static()),
        }
    }
}
