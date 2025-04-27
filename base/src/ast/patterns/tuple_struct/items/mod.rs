use crate::{ast::Pattern, Token};

mod parse;
mod to_static;
mod to_tokens;

/// The items which make up a slice pattern
#[derive(Debug, Clone)]
pub struct TupleStructItems<'a> {
    /// The first item in the struct
    pub first: Box<Pattern<'a>>,

    /// The remaining items in the struct and their separators
    pub remaining: Vec<(Token![,], Pattern<'a>)>,

    /// A last optional comma
    pub last: Option<Token![,]>,
}
