use crate::ast::patterns::{SlicePattern, SlicePatternItems};

impl<'a> SlicePattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> SlicePattern<'static> {
        SlicePattern {
            items: self.items.map(SlicePatternItems::into_static),
        }
    }
}
