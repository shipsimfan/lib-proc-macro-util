use crate::ast::TypePath;

impl<'a> TypePath<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TypePath<'static> {
        TypePath {
            leading: self.leading,
            first: self.first.into_static(),
            remaining: self
                .remaining
                .into_iter()
                .map(|(separator, segment)| (separator, segment.into_static()))
                .collect(),
        }
    }
}
