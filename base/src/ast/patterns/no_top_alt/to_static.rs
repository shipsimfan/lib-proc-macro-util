use crate::ast::PatternNoTopAlt;

impl<'a> PatternNoTopAlt<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> PatternNoTopAlt<'static> {
        match self {
            PatternNoTopAlt::Range(range) => PatternNoTopAlt::Range(range.into_static()),
            PatternNoTopAlt::WithoutRange(pattern) => {
                PatternNoTopAlt::WithoutRange(pattern.into_static())
            }
        }
    }
}
