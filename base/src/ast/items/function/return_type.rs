use crate::{ast::Type, Generator, Parse, Parser, Result, ToTokens, Token};

/// The type that is returned from a function
#[derive(Debug, Clone)]
pub struct FunctionReturnType {
    /// The right arrow introducing the type
    pub right_arrow: Token![->],

    /// The type itself
    pub r#type: Type,
}

impl<'a> Parse<'a> for FunctionReturnType {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let right_arrow = parser.parse()?;
        let r#type = parser.parse()?;

        Ok(FunctionReturnType {
            right_arrow,
            r#type,
        })
    }
}

impl ToTokens for FunctionReturnType {
    fn to_tokens(&self, generator: &mut Generator) {
        self.right_arrow.to_tokens(generator);
        self.r#type.to_tokens(generator);
    }
}
