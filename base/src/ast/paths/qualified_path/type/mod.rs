use crate::{
    ast::{Type, TypePath},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// A qualified type
#[derive(Debug, Clone)]
pub struct QualifiedPathType<'a> {
    /// The marker beginning the qualified type
    pub start: Token![<],

    /// The type to be qualified
    pub r#type: Box<Type<'a>>,

    /// The qualifier
    pub r#as: Option<(Token![as], TypePath<'a>)>,

    /// The marker of the end of the qualified type
    pub end: Token![>],
}
