use crate::{ast::patterns::RangePatternBound, Token};

mod parse;
mod to_static;
mod to_tokens;

/// A range pattern with a defined upper bound but not defined lower bound, including the upper
/// bound in the range
#[derive(Debug, Clone)]
pub struct RangeToInclusivePattern<'a> {
    /// The separator indicating this range is inclusive
    pub dots: Token![..=],

    /// The upper bound
    pub upper: RangePatternBound<'a>,
}
