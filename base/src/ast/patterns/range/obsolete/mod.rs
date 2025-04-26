use crate::{ast::patterns::RangePatternBound, Token};

mod to_static;
mod to_tokens;

/// An obsolete form of the range operator
#[derive(Debug, Clone)]
pub struct ObsoleteRangePattern<'a> {
    /// The lower bound of the range
    pub lower: RangePatternBound<'a>,

    /// The obsolete separator
    pub dots: Token![...],

    /// The upper bound
    pub upper: RangePatternBound<'a>,
}
