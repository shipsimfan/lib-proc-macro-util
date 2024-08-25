use proc_macro::Delimiter;

use super::Punctuated;
use crate::{
    ast::Path, tokens::Identifier, Error, Generator, Parse, Parser, Result, ToTokens, Token,
};

/// A Rust type
#[derive(Debug, Clone)]
pub enum Type {
    /// A type that is infered by context
    Inference(Token![_]),

    /// A path to a type
    Path(Path),

    /// A set of types
    Tuple(Punctuated<Type, Token![,]>),
}

impl<'a> Parse<'a> for Type {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![_]>() {
            Ok(Type::Inference(parser.parse()?))
        } else if let Some(group) = parser.group() {
            match group.delimiter() {
                Delimiter::Parenthesis => Ok(Type::Tuple(Punctuated::parse(
                    &mut group.tokens(),
                    false,
                    true,
                )?)),
                _ => Err(Error::new_at("expected a tuple", group.span())),
            }
        } else {
            Ok(Type::Path(parser.parse()?))
        }
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, generator: &mut Generator) {
        match self {
            Type::Inference(underscore) => underscore.to_tokens(generator),
            Type::Path(path) => path.to_tokens(generator),
            Type::Tuple(types) => types.to_tokens(&mut generator.group(Delimiter::Parenthesis)),
        }
    }
}

impl From<Identifier> for Type {
    fn from(identifier: Identifier) -> Self {
        Type::Path(identifier.into())
    }
}
