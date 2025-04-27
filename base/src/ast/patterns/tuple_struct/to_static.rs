use crate::ast::patterns::{TupleStructItems, TupleStructPattern};

impl<'a> TupleStructPattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TupleStructPattern<'static> {
        TupleStructPattern {
            path: self.path.into_static(),
            items: self.items.map(TupleStructItems::into_static),
        }
    }
}
