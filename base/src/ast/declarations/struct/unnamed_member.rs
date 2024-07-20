use crate::{
    ast::{Type, Visibility},
    Generator, Parse, Parser, Result, ToTokens,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::ast::StructDeclaration;

/// An unnamed member of a [`Struct`]
#[derive(Clone)]
pub struct UnnamedStructMember<'a> {
    /// The visibility of this member
    pub visibility: Option<Visibility<'a>>,

    /// The type of this member
    pub r#type: Type,
}

impl<'a> Parse<'a> for UnnamedStructMember<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let visibility = parser.parse()?;
        let r#type = parser.parse()?;

        Ok(UnnamedStructMember { visibility, r#type })
    }
}

impl<'a> ToTokens for UnnamedStructMember<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        self.visibility.to_tokens(generator);
        self.r#type.to_tokens(generator);
    }
}
