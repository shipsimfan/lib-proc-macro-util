use crate::{ast::Path, tokens::Identifier, Generator, Parse, Parser, Result, ToTokens, Token};

/// A Rust type
#[derive(Clone)]
pub enum Type {
    /// A type that is infered by context
    Inference(Token![_]),

    /// A path to a type
    Path(Path),
}

impl<'a> Parse<'a> for Type {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![_]>() {
            Ok(Type::Inference(parser.parse()?))
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
        }
    }
}

impl From<Identifier> for Type {
    fn from(identifier: Identifier) -> Self {
        Type::Path(identifier.into())
    }
}
