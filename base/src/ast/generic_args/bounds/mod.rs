use crate::{
    ast::{GenericArgs, TypeParamBounds},
    tokens::Identifier,
    Token,
};

pub struct GenericArgsBounds<'a> {
    pub identifier: Identifier,
    pub args: Option<Box<GenericArgs<'a>>>,
    pub colon: Token![:],
    pub bounds: TypeParamBounds<'a>,
}
