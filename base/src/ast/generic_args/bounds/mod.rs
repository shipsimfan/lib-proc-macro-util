use crate::{
    ast::{GenericArgs, TypeParamBounds},
    tokens::Identifier,
    Token,
};

mod parse;
mod to_tokens;

#[derive(Debug, Clone)]
pub struct GenericArgsBounds<'a> {
    pub identifier: Identifier,
    pub args: Option<Box<GenericArgs<'a>>>,
    pub colon: Token![:],
    pub bounds: TypeParamBounds<'a>,
}
