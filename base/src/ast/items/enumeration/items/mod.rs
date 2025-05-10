use crate::{ast::items::EnumItem, Token};

mod parse;
mod to_static;
mod to_tokens;

/// The items which make up the variants of an enum
#[derive(Debug, Clone)]
pub struct EnumItems<'a> {
    /// The first variant
    pub first: EnumItem<'a>,

    /// The remaining variants and their separators
    pub remaining: Vec<(Token![,], EnumItem<'a>)>,

    /// A final trailing comma
    pub last: Option<Token![,]>,
}
