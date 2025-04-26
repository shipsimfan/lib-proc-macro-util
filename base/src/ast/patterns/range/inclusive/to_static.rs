use crate::ast::patterns::RangeInclusivePattern;

impl<'a> RangeInclusivePattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> RangeInclusivePattern<'static> {
        RangeInclusivePattern {
            lower: self.lower.into_static(),
            dots: self.dots,
            upper: self.upper.into_static(),
        }
    }
}
