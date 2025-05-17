use crate::{ast::expressions::ClosureParam, Token};

mod parse;
mod to_static;
mod to_tokens;

/// Parameters passed into a closure
#[derive(Debug, Clone)]
pub struct ClosureParameters<'a> {
    /// The first parameter passed to the closure
    pub first: ClosureParam<'a>,

    /// The remaining parameters and their separators
    pub remaining: Vec<(Token![,], ClosureParam<'a>)>,

    /// A final optional comma
    pub last: Option<Token![,]>,
}
