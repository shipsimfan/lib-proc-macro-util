use crate::ast::patterns::ObsoleteRangePattern;

impl<'a> ObsoleteRangePattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ObsoleteRangePattern<'static> {
        ObsoleteRangePattern {
            lower: self.lower.into_static(),
            dots: self.dots,
            upper: self.upper.into_static(),
        }
    }
}
