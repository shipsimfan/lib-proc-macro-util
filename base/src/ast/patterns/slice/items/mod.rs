use crate::{ast::Pattern, Token};

mod parse;
mod to_static;
mod to_tokens;

/// The items which make up a slice pattern
#[derive(Debug, Clone)]
pub struct SlicePatternItems<'a> {
    /// The first item in the slice
    pub first: Box<Pattern<'a>>,

    /// The remaining items in the slice
    pub remaining: Vec<(Token![,], Pattern<'a>)>,

    /// An optional last comma
    pub last: Option<Token![,]>,
}
