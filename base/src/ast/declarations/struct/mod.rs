use crate::{
    ast::{Generics, Visibility},
    tokens::Identifier,
};

mod body;
mod named_member;
mod unnamed_member;

pub use body::StructBody;
pub use named_member::NamedStructMember;
pub use unnamed_member::UnnamedStructMember;

/// A declaration defining a structure
///
/// Example: `struct Example { a: u8 }`
#[derive(Clone)]
pub struct Struct<'a> {
    /// The visibility of this structure
    pub visibility: Option<Visibility<'a>>,

    /// The `struct` token itself
    pub r#struct: crate::tokens::Struct,

    /// The [`Identifier`] naming this structure
    pub identifier: Identifier,

    /// The generics for this structure
    pub generics: Option<Generics>,

    /// The body of the structure
    pub body: StructBody<'a>,
}
