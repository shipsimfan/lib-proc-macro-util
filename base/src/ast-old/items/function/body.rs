use crate::{tokens::Group, Error, Generator, Parse, Parser, Result, ToTokens};
use proc_macro::Delimiter;

/// A body of a function
#[derive(Debug, Clone)]
pub struct FunctionBody<'a>(pub Group<'a>);

impl<'a> Parse<'a> for FunctionBody<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: Group<'a> = parser.parse()?;

        if group.delimiter() != Delimiter::Brace {
            return Err(Error::new_at(
                "function body must be surrounded with braces",
                group.span(),
            ));
        }

        Ok(FunctionBody(group))
    }
}

impl<'a> ToTokens for FunctionBody<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        self.0.to_tokens(generator);
    }
}
