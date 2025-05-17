use crate::Token;

mod parse;
mod to_tokens;

/// The type of range being defined
#[derive(Debug, Clone)]
pub enum RangeOperator {
    /// The range does not include the upper limit
    Exclusive(Token![..]),

    /// The range includes the upper limit
    Inclusive(Token![..=]),
}
