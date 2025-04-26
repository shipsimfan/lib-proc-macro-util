use crate::ast::patterns::RangeExclusivePattern;

impl<'a> RangeExclusivePattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> RangeExclusivePattern<'static> {
        RangeExclusivePattern {
            lower: self.lower.into_static(),
            dots: self.dots,
            upper: self.upper.into_static(),
        }
    }
}
