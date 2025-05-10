use crate::{
    ast::{OuterAttribute, Visibility},
    tokens::Identifier,
};
use std::borrow::Cow;

mod discriminant;
mod kind;

mod parse;
mod to_static;
mod to_tokens;

pub use discriminant::EnumItemDiscriminant;
pub use kind::EnumItemKind;

/// A variant in an enum
#[derive(Debug, Clone)]
pub struct EnumItem<'a> {
    /// The attributes effecting this variant
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The visibility of this variant
    pub visibility: Option<Visibility<'a>>,

    /// The name of this variant
    pub name: Cow<'a, Identifier>,

    /// The types associated this enum
    pub kind: Option<EnumItemKind<'a>>,

    /// The discriminant defining the value of this variant
    pub discriminant: Option<EnumItemDiscriminant<'a>>,
}
