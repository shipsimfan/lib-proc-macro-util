use crate::ast::Pattern;

impl<'a> Pattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> Pattern<'static> {
        Pattern {
            leading: self.leading,
            first: self.first.into_static(),
            remaining: self
                .remaining
                .into_iter()
                .map(|(or, pattern)| (or, pattern.into_static()))
                .collect(),
        }
    }
}
