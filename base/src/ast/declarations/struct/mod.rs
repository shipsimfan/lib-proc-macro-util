use crate::{
    ast::{Generics, Meta, Visibility},
    tokens::Identifier,
    Generator, Parser, Result, ToTokens,
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
pub struct StructDeclaration<'a> {
    /// The metadata about the structure
    pub meta: Vec<Meta<'a>>,

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

impl<'a> StructDeclaration<'a> {
    /// Parses the [`Struct`] following `visibility`
    pub fn parse(
        meta: Vec<Meta<'a>>,
        visibility: Option<Visibility<'a>>,
        parser: &mut Parser<'a>,
    ) -> Result<Self> {
        let r#struct = parser.parse()?;
        let identifier = parser.parse()?;
        let generics = parser.parse()?;
        let body = parser.parse()?;

        Ok(StructDeclaration {
            meta,
            visibility,
            r#struct,
            identifier,
            generics,
            body,
        })
    }
}

impl<'a> ToTokens for StructDeclaration<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        self.visibility.to_tokens(generator);
        self.r#struct.to_tokens(generator);
        self.identifier.to_tokens(generator);
        self.generics.to_tokens(generator);
        self.body.to_tokens(generator);
    }
}
