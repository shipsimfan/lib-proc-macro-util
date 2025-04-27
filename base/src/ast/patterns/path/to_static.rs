use crate::ast::patterns::PathPattern;

impl<'a> PathPattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> PathPattern<'static> {
        PathPattern {
            path: self.path.into_static(),
        }
    }
}
