use crate::{ast::GenericArguments, tokens::Identifier, Parse, ToTokens};

/// A single segment of a path
#[derive(Clone)]
pub struct PathSegment {
    /// The identifier
    pub identifier: Identifier,

    /// Generic arguments for the identifier
    pub arguments: Option<GenericArguments>,
}

impl<'a> Parse<'a> for PathSegment {
    fn parse(parser: &mut crate::Parser<'a>) -> crate::Result<Self> {
        let identifier = parser.parse()?;
        let arguments = parser.parse()?;

        Ok(PathSegment {
            identifier,
            arguments,
        })
    }
}

impl ToTokens for PathSegment {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        self.identifier.to_tokens(generator);
        self.arguments.to_tokens(generator);
    }
}

impl From<Identifier> for PathSegment {
    fn from(identifier: Identifier) -> Self {
        PathSegment {
            identifier,
            arguments: None,
        }
    }
}
