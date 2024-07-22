use crate::{Error, Generator, Parse, Parser, Result, ToTokens};
use proc_macro::Span;

/// An identifier
#[derive(Debug, Clone)]
pub struct Identifier(proc_macro::Ident);

impl Identifier {
    /// Creates a new [`Identifier`]
    ///
    /// ## Parameters
    ///  * `identifier` - The identifier
    ///  * `span` - The [`Span`] for the new identifier
    ///
    /// ## Return Value
    /// Returns the newly created [`Identifier`]
    pub fn new(identifier: &str, span: Span) -> Self {
        Identifier(proc_macro::Ident::new(identifier, span))
    }

    /// Get the [`Span`] of this identifier
    ///
    /// ## Return Value
    /// Returns this identifier's [`Span`]
    pub fn span(&self) -> Span {
        self.0.span()
    }

    /// Converts this identifier to a [`String`]
    ///
    /// ## Return Value
    /// Returns the [`String`] representation of this identifier
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<T: AsRef<str>> PartialEq<T> for Identifier {
    fn eq(&self, other: &T) -> bool {
        self.0.to_string() == other.as_ref()
    }
}

impl From<proc_macro::Ident> for Identifier {
    fn from(ident: proc_macro::Ident) -> Self {
        Identifier(ident)
    }
}

impl Into<proc_macro::Ident> for Identifier {
    fn into(self) -> proc_macro::Ident {
        self.0
    }
}

impl<'a> Parse<'a> for Identifier {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser
            .identifier()
            .ok_or(Error::new("expected an identifier"))
    }
}

impl ToTokens for Identifier {
    fn to_tokens(&self, generator: &mut Generator) {
        generator.identifier_string_at(&self.0.to_string(), self.0.span())
    }
}
