use crate::ast::patterns::RangePattern;

impl<'a> RangePattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> RangePattern<'static> {
        match self {
            RangePattern::Exclusive(exclusive) => RangePattern::Exclusive(exclusive.into_static()),
            RangePattern::Inclusive(inclusive) => RangePattern::Inclusive(inclusive.into_static()),
            RangePattern::From(from) => RangePattern::From(from.into_static()),
            RangePattern::ToInclusive(to_inclusive) => {
                RangePattern::ToInclusive(to_inclusive.into_static())
            }
            RangePattern::Obsolete(obsolete) => RangePattern::Obsolete(obsolete.into_static()),
        }
    }
}
