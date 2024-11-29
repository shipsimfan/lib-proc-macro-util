use crate::ast::types::ArrayType;

impl<'a> ArrayType<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ArrayType<'static> {
        ArrayType {
            r#type: Box::new(self.r#type.into_static()),
            semi: self.semi,
            count: self.count.into_static(),
        }
    }
}
