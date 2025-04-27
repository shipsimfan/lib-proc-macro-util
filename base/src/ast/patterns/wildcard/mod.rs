use crate::Token;

mod parse;
mod to_tokens;

/// The wildcard pattern (an underscore symbol) matches any value.
#[derive(Debug, Clone)]
pub struct WildcardPattern {
    /// The underscore indicating this is a wildcard pattern
    pub underscore: Token![_],
}
