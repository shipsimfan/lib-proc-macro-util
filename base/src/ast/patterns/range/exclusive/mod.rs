use crate::{ast::patterns::RangePatternBound, Token};

mod to_static;
mod to_tokens;

/// A range pattern with a defined lower and upper bound, excluding the upper bound in the range
#[derive(Debug, Clone)]
pub struct RangeExclusivePattern<'a> {
    /// The lower bound of the range
    pub lower: RangePatternBound<'a>,

    /// The separator indicating this range is exclusive
    pub dots: Token![..],

    /// The upper bound
    pub upper: RangePatternBound<'a>,
}
