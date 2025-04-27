use crate::ast::patterns::{StructPattern, StructPatternElements};

impl<'a> StructPattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructPattern<'static> {
        StructPattern {
            path: self.path.into_static(),
            elements: self.elements.map(StructPatternElements::into_static),
        }
    }
}
