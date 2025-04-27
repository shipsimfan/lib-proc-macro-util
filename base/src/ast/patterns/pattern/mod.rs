use crate::{ast::PatternNoTopAlt, Token};

mod parse;
mod to_static;
mod to_tokens;

/// A set of patterns
#[derive(Debug, Clone)]
pub struct Pattern<'a> {
    /// An optional leading vertical bar
    pub leading: Option<Token![|]>,

    /// The first required pattern
    pub first: PatternNoTopAlt<'a>,

    /// The remaining patterns and their separators
    pub remaining: Vec<(Token![|], PatternNoTopAlt<'a>)>,
}
