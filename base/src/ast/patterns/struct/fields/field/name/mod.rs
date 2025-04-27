use crate::{
    ast::Pattern,
    tokens::{Identifier, Literal},
    Token,
};
use std::borrow::Cow;

mod parse;
mod to_static;
mod to_tokens;

/// The name of a field in a struct pattern
#[derive(Debug, Clone)]
pub enum StructPatternFieldName<'a> {
    /// The field is named by an index
    Index(Cow<'a, Literal>, Token![:], Box<Pattern<'a>>),

    /// The field is named by an identifier with a pattern
    IdentifierPattern(Cow<'a, Identifier>, Token![:], Box<Pattern<'a>>),

    /// The field is named by an identified without a pattern
    Identifier(
        Option<Token![ref]>,
        Option<Token![mut]>,
        Cow<'a, Identifier>,
    ),
}
