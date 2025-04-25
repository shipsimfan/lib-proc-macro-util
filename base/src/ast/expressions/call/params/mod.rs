use crate::{ast::Expression, Token};

mod parse;
mod to_static;
mod to_tokens;

/// The parameters to a function call
#[derive(Debug, Clone)]
pub struct CallParams<'a> {
    /// The first parameter
    pub first: Option<Box<Expression<'a>>>,

    /// The remaining parameters and their leadining commas
    pub remaining: Vec<(Token![,], Expression<'a>)>,

    /// A final trailing comma
    pub last: Option<Token![,]>,
}
