use crate::{tokens::Identifier, Generator, Parse, Parser, Result, ToTokens, Token};

/// A variable name
pub enum VariableName {
    /// The variable has an identifier
    Identifier(Identifier),

    /// The variable has no identifier
    Underscore(Token![_]),
}

impl<'a> Parse<'a> for VariableName {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![_]>() {
            parser
                .parse()
                .map(|underscore| VariableName::Underscore(underscore))
        } else {
            parser
                .parse()
                .map(|identifer| VariableName::Identifier(identifer))
        }
    }
}

impl ToTokens for VariableName {
    fn to_tokens(&self, generator: &mut Generator) {
        match self {
            VariableName::Identifier(identifier) => identifier.to_tokens(generator),
            VariableName::Underscore(underscore) => underscore.to_tokens(generator),
        }
    }
}
