mod bound;
mod exclusive;
mod from;
mod inclusive;
mod obsolete;
mod to_inclusive;

mod parse;
mod to_static;
mod to_tokens;

pub use bound::RangePatternBound;
pub use exclusive::RangeExclusivePattern;
pub use from::RangeFromPattern;
pub use inclusive::RangeInclusivePattern;
pub use obsolete::ObsoleteRangePattern;
pub use to_inclusive::RangeToInclusivePattern;

/// Range patterns match scalar values within the range defined by their bounds.
#[derive(Debug, Clone)]
pub enum RangePattern<'a> {
    /// A full range which excludes the upper bound
    Exclusive(RangeExclusivePattern<'a>),

    /// A full range which includes the upper bound
    Inclusive(RangeInclusivePattern<'a>),

    /// A range with no defined upper bound
    From(RangeFromPattern<'a>),

    /// A range with no defined lower bound which includes the upper bound
    ToInclusive(RangeToInclusivePattern<'a>),

    /// An obsolete form of the range operator
    Obsolete(ObsoleteRangePattern<'a>),
}
