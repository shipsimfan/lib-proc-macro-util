use crate::ast::patterns::TupleStructItems;

impl<'a> TupleStructItems<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TupleStructItems<'static> {
        TupleStructItems {
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
