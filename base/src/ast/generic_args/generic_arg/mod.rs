use crate::{
    ast::{GenericArgsBinding, GenericArgsBounds, GenericArgsConst, Lifetime},
    tokens::Type,
};

mod parse;
mod to_tokens;

/// A generic argument
#[derive(Debug, Clone)]
pub enum GenericArg<'a> {
    /// The argument is a lifetime
    Lifetime(Lifetime<'a>),

    /// The argument is a type
    Type(Type),

    /// The argument is constant
    Const(GenericArgsConst<'a>),

    /// The argument has a value
    Binding(GenericArgsBinding<'a>),

    /// The argument has bounds
    Bounds(GenericArgsBounds<'a>),
}
