use crate::ast::{
    types::{
        ImplTraitTypeOneBound, NeverType, ParenthesizedType, TraitObjectTypeOneBound, TupleType,
    },
    TypePath,
};

mod parse;
mod to_tokens;

/// A type which can be defined without bounds
#[derive(Debug, Clone)]
pub enum TypeNoBounds<'a> {
    /// The type is surrounded by parentheses
    Parenthesized(ParenthesizedType<'a>),

    /// An unnamed type implementing one trait
    ImplTraitOneBound(ImplTraitTypeOneBound<'a>),

    /// An opaque value of another type that a trait
    TraitObjectOneBound(TraitObjectTypeOneBound<'a>),

    /// A path to a type
    Path(TypePath<'a>),

    /// An ordered heterogenous list of types
    Tuple(TupleType<'a>),

    /// A type which has no value and represents computation that never completes
    Never(NeverType),
}
