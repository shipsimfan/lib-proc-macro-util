use crate::{
    ast::{QualifiedPathType, TypePathSegment},
    Token,
};

mod parse;
mod to_tokens;

/// A qualified path in a type
#[derive(Debug, Clone)]
pub struct QualifiedPathInType<'a> {
    /// The base type
    pub r#type: QualifiedPathType<'a>,

    /// The path to a sub-item with separators
    pub segments: Vec<(Token![::], TypePathSegment<'a>)>,
}
