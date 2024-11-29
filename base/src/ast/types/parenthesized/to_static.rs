use crate::ast::types::ParenthesizedType;

impl<'a> ParenthesizedType<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ParenthesizedType<'static> {
        ParenthesizedType {
            r#type: Box::new(self.r#type.into_static()),
        }
    }
}
