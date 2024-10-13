use crate::ast::types::{
    ImplTraitType, ImplTraitTypeOneBound, ParenthesizedType, TraitObjectType,
    TraitObjectTypeOneBound,
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
}