use crate::{
    ast::{Type, Visibility},
    tokens::Identifier,
    Generator, Parse, Parser, Result, ToTokens, Token,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::ast::StructDeclaration;

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

impl<'a> Parse<'a> for NamedStructMember<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let visibility = parser.parse()?;
        let name = parser.parse()?;
        let colon = parser.parse()?;
        let r#type = parser.parse()?;

        Ok(NamedStructMember {
            visibility,
            name,
            colon,
            r#type,
        })
    }
}

impl<'a> ToTokens for NamedStructMember<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        self.visibility.to_tokens(generator);
        self.name.to_tokens(generator);
        self.colon.to_tokens(generator);
        self.r#type.to_tokens(generator);
    }
}
