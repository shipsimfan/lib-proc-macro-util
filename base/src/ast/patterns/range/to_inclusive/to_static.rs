use crate::ast::patterns::RangeToInclusivePattern;

impl<'a> RangeToInclusivePattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> RangeToInclusivePattern<'static> {
        RangeToInclusivePattern {
            dots: self.dots,
            upper: self.upper.into_static(),
        }
    }
}
