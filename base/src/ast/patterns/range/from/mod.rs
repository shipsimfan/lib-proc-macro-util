use crate::{ast::patterns::RangePatternBound, Token};

mod to_static;
mod to_tokens;

/// A range pattern with a defined lower but not a defined upper bound
#[derive(Debug, Clone)]
pub struct RangeFromPattern<'a> {
    /// The lower bound of the range
    pub lower: RangePatternBound<'a>,

    /// The separator indicating this is a ranges
    pub dots: Token![..],
}
