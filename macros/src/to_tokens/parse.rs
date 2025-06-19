use super::{ToTokens, Token};
use proc_macro_util_base::{tokens::Identifier, Parse, Parser, Result};

impl<'a> ToTokens<'a> {
    /// Attempts to parse the `to_tokens` macro from `parser`, with the output using `base` as the
    /// path to the `proc_macro_util` crate. This will not try to parse the generator name.
    pub fn parse_without_name(parser: &mut Parser<'a>, generator: &'a Identifier) -> Result<Self> {
        let mut tokens = Vec::new();
        while !parser.empty() {
            tokens.push(Token::parse(parser, generator)?)
        }

        Ok(ToTokens { generator, tokens })
    }
}

impl<'a> Parse<'a> for ToTokens<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let generator = parser
            .parse()
            .map_err(|error| error.error("expected the name of the generator to use"))?;

        ToTokens::parse_without_name(parser, generator)
    }
}
