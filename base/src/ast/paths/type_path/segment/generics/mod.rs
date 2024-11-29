use crate::ast::{GenericArgs, TypePathFn};

mod parse;
mod to_static;
mod to_tokens;

/// A set of generics which can modify a segment in a type path
#[derive(Debug, Clone)]
pub enum TypePathSegmentGenerics<'a> {
    /// Generic arguments
    GenericArgs(GenericArgs<'a>),

    /// A function
    TypePathFn(TypePathFn<'a>),
}
