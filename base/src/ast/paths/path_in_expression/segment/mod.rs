use crate::{
    ast::{GenericArgs, PathIdentSegment},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// A single segment of a path in an expression
#[derive(Debug, Clone)]
pub struct PathExprSegment<'a> {
    /// The name of the segment
    pub ident: PathIdentSegment,

    /// Generic arugments modifying this segment
    pub generic_args: Option<(Token![::], GenericArgs<'a>)>,
}
