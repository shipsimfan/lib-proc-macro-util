use crate::Token;

mod segment;

mod parse;
mod to_tokens;

pub use segment::PathExprSegment;

/// A path in an expression
#[derive(Debug, Clone)]
pub struct PathInExpression<'a> {
    /// The marker if this path is absolute or relative
    pub leading: Option<Token![::]>,

    /// The first required segment of the path
    pub first: PathExprSegment<'a>,

    /// The remaining segments and their separators
    pub remaining: Vec<(Token![::], PathExprSegment<'a>)>,
}
