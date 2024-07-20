use crate::{
    ast::{NamedStructMember, Punctuated, UnnamedStructMember},
    Error, Generator, Parse, Parser, Result, ToTokens, Token,
};
use proc_macro::Delimiter;

// rustdoc imports
#[allow(unused_imports)]
use crate::ast::StructDeclaration;

/// The body of a [`Struct`]
#[derive(Clone)]
pub enum StructBody<'a> {
    /// The struct has no body
    None(Token![;]),

    /// The struct constists of unnamed members
    Unnamed(Punctuated<UnnamedStructMember<'a>, Token![,]>, Token![;]),

    /// The struct consists of named members
    Named(Punctuated<NamedStructMember<'a>, Token![,]>),
}

impl<'a> Parse<'a> for StructBody<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![;]>() {
            return parser.parse().map(|semicolon| StructBody::None(semicolon));
        }

        let group = parser
            .group()
            .ok_or(Error::new_at("expected a struct body", parser.span()))?;

        match group.delimiter() {
            Delimiter::Parenthesis => Ok(StructBody::Unnamed(
                Punctuated::parse(&mut group.tokens(), false, true)?,
                parser.parse()?,
            )),
            Delimiter::Brace => Punctuated::parse(&mut group.tokens(), false, true)
                .map(|members| StructBody::Named(members)),
            _ => Err(Error::new_at(
                "expected a braces or parenthesis",
                group.span(),
            )),
        }
    }
}

impl<'a> ToTokens for StructBody<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        match self {
            StructBody::None(semicolon) => semicolon.to_tokens(generator),
            StructBody::Unnamed(members, semicolon) => {
                let mut group = generator.group(Delimiter::Parenthesis);
                members.to_tokens(&mut group);
                semicolon.to_tokens(generator);
            }
            StructBody::Named(members) => {
                let mut group = generator.group(Delimiter::Brace);
                members.to_tokens(&mut group);
            }
        }
    }
}
