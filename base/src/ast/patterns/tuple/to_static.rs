use crate::ast::patterns::{TuplePattern, TuplePatternItems};

impl<'a> TuplePattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TuplePattern<'static> {
        TuplePattern {
            items: self.items.map(TuplePatternItems::into_static),
        }
    }
}
