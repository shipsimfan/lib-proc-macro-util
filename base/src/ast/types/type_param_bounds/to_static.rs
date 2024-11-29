use crate::ast::TypeParamBounds;

impl<'a> TypeParamBounds<'a> {
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
