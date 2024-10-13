use crate::ast::{
    types::{
        ArrayType, ImplTraitType, ImplTraitTypeOneBound, NeverType, ParenthesizedType,
        RawPointerType, ReferenceType, SliceType, TraitObjectType, TraitObjectTypeOneBound,
        TupleType,
    },
    TypePath,
};

mod parse;
mod to_tokens;

/// An syntax element referencing a type
#[derive(Debug, Clone)]
pub enum Type<'a> {
    /// The type is surrounded by parentheses
    Parenthesized(ParenthesizedType<'a>),

    /// An unnamed type implementing one or more traits
    ImplTrait(ImplTraitType<'a>),

    /// An unnamed type implementing one trait
    ImplTraitOneBound(ImplTraitTypeOneBound<'a>),

    /// An opaque value of another type that implements a set of traits
    TraitObject(TraitObjectType<'a>),

    /// An opaque value of another type that a trait
    TraitObjectOneBound(TraitObjectTypeOneBound<'a>),

    /// A path to a type
    Path(TypePath<'a>),

    /// An ordered heterogenous list of types
    Tuple(TupleType<'a>),

    /// A type which has no value and represents computation that never completes
    Never(NeverType),

    /// A raw pointer to another type
    RawPointer(RawPointerType<'a>),

    /// A reference to another type
    Reference(ReferenceType<'a>),

    /// A fixed-sized homogenous sequence of values
    Array(ArrayType<'a>),

    /// A dynamically-sized homogenous sequence of values
    Slice(SliceType<'a>),
}
