use crate::ast::types::SliceType;

impl<'a> SliceType<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> SliceType<'static> {
        SliceType {
            r#type: Box::new(self.r#type.into_static()),
        }
    }
}
