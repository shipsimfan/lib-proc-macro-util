use crate::Token;

mod r#fn;
mod segment;

pub use r#fn::{TypePathFn, TypePathFnInputs};
pub use segment::{TypePathSegment, TypePathSegmentGenerics};

pub struct TypePath<'a> {
    pub leading: Option<Token![::]>,
    pub first: TypePathSegment<'a>,
    pub remaining: Vec<(Token![::], TypePathSegment<'a>)>,
}
