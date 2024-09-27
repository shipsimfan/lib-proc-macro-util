use crate::{
    ast::{GenericArgsBinding, GenericArgsBounds, GenericArgsConst, Lifetime},
    tokens::Type,
};

mod parse;
mod to_tokens;

#[derive(Debug, Clone)]
pub enum GenericArg<'a> {
    Lifetime(Lifetime<'a>),
    Type(Type),
    Const(GenericArgsConst<'a>),
    Binding(GenericArgsBinding<'a>),
    Bounds(GenericArgsBounds<'a>),
}
