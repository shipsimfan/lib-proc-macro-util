use crate::ast::TypeParamBounds;

impl<'a> TypeParamBounds<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TypeParamBounds<'static> {
        TypeParamBounds {
            first: self.first.into_static(),
            remaining: self
                .remaining
                .into_iter()
                .map(|(plus, bound)| (plus, bound.into_static()))
                .collect(),
            end: self.end,
        }
    }
}
