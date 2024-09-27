use crate::ast::{GenericArgs, TypePathFn};

pub enum TypePathSegmentGenerics<'a> {
    GenericArgs(GenericArgs<'a>),
    TypePathFn(TypePathFn),
}
