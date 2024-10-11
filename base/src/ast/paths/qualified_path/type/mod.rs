use crate::{ast::TypePath, tokens::Type, Token};

mod parse;
mod to_tokens;

/// A qualified type
#[derive(Debug, Clone)]
pub struct QualifiedPathType<'a> {
    /// The marker beginning the qualified type
    pub start: Token![<],

    /// The type to be qualified
    pub r#type: Type,

    /// The qualifier
    pub r#as: Option<(Token![as], TypePath<'a>)>,

    /// The marker of the end of the qualified type
    pub end: Token![>],
}
