use crate::ast::patterns::ReferencePattern;

impl<'a> ReferencePattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ReferencePattern<'static> {
        ReferencePattern {
            ampersand: self.ampersand,
            r#mut: self.r#mut,
            pattern: Box::new(self.pattern.into_static()),
        }
    }
}
