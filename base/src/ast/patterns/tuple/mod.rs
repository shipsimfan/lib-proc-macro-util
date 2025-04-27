mod items;

mod parse;
mod to_static;
mod to_tokens;

pub use items::TuplePatternItems;

/// Tuple patterns match tuple values that match all criteria defined by its subpatterns.
#[derive(Debug, Clone)]
pub struct TuplePattern<'a> {
    /// The items the be matched in the tuple
    pub items: Option<TuplePatternItems<'a>>,
}
