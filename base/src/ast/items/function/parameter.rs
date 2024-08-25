use crate::{
    ast::{Type, VariableName},
    Generator, Parse, Parser, Result, ToTokens, Token,
};

/// A parameter passed to a function
#[derive(Debug, Clone)]
pub struct FunctionParameter {
    /// The name of the parameter
    pub name: VariableName,

    /// The colon seperating the type
    pub colon: Token![:],

    /// The type of the parameter
    pub r#type: Type,
}

impl<'a> Parse<'a> for FunctionParameter {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let name = parser.parse()?;
        let colon = parser.parse()?;
        let r#type = parser.parse()?;

        Ok(FunctionParameter {
            name,
            colon,
            r#type,
        })
    }
}

impl ToTokens for FunctionParameter {
    fn to_tokens(&self, generator: &mut Generator) {
        self.name.to_tokens(generator);
        self.colon.to_tokens(generator);
        self.r#type.to_tokens(generator);
    }
}
