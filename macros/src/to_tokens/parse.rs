use super::{ToTokens, Token};
use i18n_translation::m;
use proc_macro_util_base::{supported_languages::*, tokens::Identifier, Parse, Parser, Result};

i18n_translation::message_key!( ExpectedAttribute [
    EN => { "expected the name of the generator to use" },
    FR => { "le nom du générateur à utiliser était attendu" },
    ZH => { "预期要使用的生成器名称" },
]);

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
            .map_err(|error| error.append(m!(ExpectedAttribute)))?;

        ToTokens::parse_without_name(parser, generator)
    }
}
