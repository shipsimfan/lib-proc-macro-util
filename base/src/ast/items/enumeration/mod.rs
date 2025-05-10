use crate::{
    ast::{GenericParams, WhereClause},
    tokens::Identifier,
    Token,
};
use std::borrow::Cow;

mod item;
mod items;

mod parse;
mod to_static;
mod to_tokens;

pub use item::{EnumItem, EnumItemDiscriminant, EnumItemKind};
pub use items::EnumItems;

/// An enumeration, also referred to as an enum, is a simultaneous definition of a nominal
/// enumerated type as well as a set of constructors, that can be used to create or pattern-match
/// values of the corresponding enumerated type.
#[derive(Debug, Clone)]
pub struct Enumeration<'a> {
    /// The token identifying this item as an enum
    pub r#enum: Token![enum],

    /// The name of the enumeration
    pub name: Cow<'a, Identifier>,

    /// The generic parameters for this enum
    pub generic_params: Option<GenericParams<'a>>,

    /// The where clause restricting the generic parameters
    pub where_clause: Option<WhereClause<'a>>,

    /// The items which make up the variants of the enum
    pub enum_items: Option<EnumItems<'a>>,
}
