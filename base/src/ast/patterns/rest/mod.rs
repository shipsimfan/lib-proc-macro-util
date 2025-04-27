use crate::Token;

mod parse;
mod to_tokens;

/// The rest pattern acts as a variable-length pattern which matches zero or more elements that
/// haven’t been matched already before and after.
#[derive(Debug, Clone)]
pub struct RestPattern {
    /// The dots indicating this is a rest pattern
    pub dots: Token![..],
}
