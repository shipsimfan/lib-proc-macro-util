use crate::ast::{
    types::{
        ArrayType, BareFunctionType, ImplTraitTypeOneBound, InferredType, NeverType,
        ParenthesizedType, RawPointerType, ReferenceType, SliceType, TraitObjectTypeOneBound,
        TupleType,
    },
    MacroInvocation, QualifiedPathInType, TypePath,
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

    /// A raw pointer to another type
    RawPointer(RawPointerType<'a>),

    /// A reference to another type
    Reference(ReferenceType<'a>),

    /// A fixed-sized homogenous sequence of values
    Array(ArrayType<'a>),

    /// A dynamically-sized homogenous sequence of values
    Slice(SliceType<'a>),

    /// A type which has been explicitly specified and will be inferred by the compiler
    Inferred(InferredType),

    /// A type qualified as another type or trait
    QualifiedPath(QualifiedPathInType<'a>),

    /// A call to a macro
    MacroInvocation(MacroInvocation<'a>),

    /// A bare function
    BareFunction(BareFunctionType<'a>),
}
