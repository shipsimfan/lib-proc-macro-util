use crate::ast::patterns::PatternNoTopAlt;

impl<'a> PatternNoTopAlt<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> PatternNoTopAlt<'static> {
        match self {
            PatternNoTopAlt::Range(range) => PatternNoTopAlt::Range(range.into_static()),
        }
    }
}
