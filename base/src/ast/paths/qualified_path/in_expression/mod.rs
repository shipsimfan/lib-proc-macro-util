use crate::{
    ast::{PathExprSegment, QualifiedPathType},
    Token,
};

mod parse;
mod to_tokens;

/// A qualified path that is in an expression
#[derive(Debug, Clone)]
pub struct QualifiedPathInExpression<'a> {
    /// The base type
    pub r#type: QualifiedPathType<'a>,

    /// The path to a sub-item with separators
    pub segments: Vec<(Token![::], PathExprSegment<'a>)>,
}
