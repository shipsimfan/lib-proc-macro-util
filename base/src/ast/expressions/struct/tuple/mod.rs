use crate::{ast::Expression, Token};

mod parse;
mod to_static;
mod to_tokens;

/// A struct expression which is of a tuple-style
#[derive(Debug, Clone)]
pub struct StructExprTuple<'a> {
    /// The first element of the tuple
    pub first: Box<Expression<'a>>,

    /// The remaining elements and their separators
    pub remaining: Vec<(Token![,], Expression<'a>)>,

    /// A final optional comma
    pub last: Option<Token![,]>,
}
