use crate::ast::patterns::StructPatternFields;

impl<'a> StructPatternFields<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructPatternFields<'static> {
        StructPatternFields {
            first: self.first.into_static(),
            remaining: self
                .remaining
                .into_iter()
                .map(|(comma, pattern)| (comma, pattern.into_static()))
                .collect(),
        }
    }
}
