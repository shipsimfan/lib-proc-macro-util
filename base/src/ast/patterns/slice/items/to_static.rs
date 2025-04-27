use crate::ast::patterns::SlicePatternItems;

impl<'a> SlicePatternItems<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> SlicePatternItems<'static> {
        SlicePatternItems {
            first: Box::new(self.first.into_static()),
            remaining: self
                .remaining
                .into_iter()
                .map(|(comma, pattern)| (comma, pattern.into_static()))
                .collect(),
            last: self.last,
        }
    }
}
