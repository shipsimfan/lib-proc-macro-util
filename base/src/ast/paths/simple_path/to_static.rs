use crate::ast::SimplePath;

impl<'a> SimplePath<'a> {
    pub fn into_static(self) -> SimplePath<'static> {
        SimplePath {
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
