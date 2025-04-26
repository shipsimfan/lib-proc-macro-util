use crate::ast::patterns::RangeFromPattern;

impl<'a> RangeFromPattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> RangeFromPattern<'static> {
        RangeFromPattern {
            lower: self.lower.into_static(),
            dots: self.dots,
        }
    }
}
