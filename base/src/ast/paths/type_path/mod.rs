use crate::Token;

mod r#fn;
mod segment;

mod parse;
mod to_tokens;

pub use r#fn::{TypePathFn, TypePathFnInputs};
pub use segment::{TypePathSegment, TypePathSegmentGenerics};

/// A path to a type
#[derive(Debug, Clone)]
pub struct TypePath<'a> {
    /// A marker make this path absolute
    pub leading: Option<Token![::]>,

    /// The first required segments
    pub first: TypePathSegment<'a>,

    /// The remaining segments of the path and their separators
    pub remaining: Vec<(Token![::], TypePathSegment<'a>)>,
}
