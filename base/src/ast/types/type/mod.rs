use crate::ast::types::{ImplTraitType, ImplTraitTypeOneBound, ParenthesizedType};

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
}
