use crate::{ast::PathIdentSegment, Token};

mod generics;

pub use generics::TypePathSegmentGenerics;

pub struct TypePathSegment<'a> {
    pub ident: PathIdentSegment,
    pub colon: Option<Token![:]>,
    pub generics: TypePathSegmentGenerics<'a>,
}
