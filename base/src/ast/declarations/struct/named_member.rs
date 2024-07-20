use crate::{ast::{Type, Visibility}, tokens::Identifier, Token};

// rustdoc imports
#[allow(unused_imports)]
use crate::ast::Struct;

/// A named member of a [`Struct`]
#[derive(Clone)]
pub struct NamedStructMember<'a> {
    /// The visibility of this member
    pub visibility: Option<Visibility<'a>>,

    /// The name of this member
    pub name: Identifier,

    /// The colon seperating the name and type of this member
    pub colon: Token![:],

    /// The type of this member
    pub r#type: Type,
}