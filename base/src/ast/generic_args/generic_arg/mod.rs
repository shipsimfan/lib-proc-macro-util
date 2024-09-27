use crate::{
    ast::{GenericArgsBinding, GenericArgsBounds, GenericArgsConst, Lifetime},
    tokens::Type,
};

pub enum GenericArg<'a> {
    Lifetime(Lifetime<'a>),
    Type(Type),
    Const(GenericArgsConst<'a>),
    Binding(GenericArgsBinding<'a>),
    Bounds(GenericArgsBounds<'a>),
}
