use crate::ast::items::StructFields;

impl<'a> StructFields<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructFields<'static> {
        StructFields {
            first: self.first.into_static(),
            remaining: self
                .remaining
                .into_iter()
                .map(|(comma, field)| (comma, field.into_static()))
                .collect(),
            last: self.last,
        }
    }
}
