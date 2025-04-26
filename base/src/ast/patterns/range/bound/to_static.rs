use crate::ast::patterns::RangePatternBound;
use std::borrow::Cow;

impl<'a> RangePatternBound<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> RangePatternBound<'static> {
        match self {
            RangePatternBound::Literal(neg, literal) => RangePatternBound::Literal(
                neg,
                Cow::Owned(match literal {
                    Cow::Owned(owned) => owned,
                    Cow::Borrowed(borrowed) => borrowed.clone(),
                }),
            ),
            RangePatternBound::Path(path) => RangePatternBound::Path(path.into_static()),
        }
    }
}
