use crate::{
    supported_languages::*,
    tokens::{Identifier, TokenTree},
    Parse, Parser, Result,
};
use i18n_translation::m;

i18n_translation::message_key!( ExpectedIdentifier [
    EN => { "expected an identifier" },
    FR => { "un identifiant était attendu" },
    ZH => { "预期的标识符" },
]);

impl<'a> Parse<'a> for &'a Identifier {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        match parser.next() {
            Some(TokenTree::Identifier(identifier)) => Ok(identifier.into()),
            _ => Err(parser.error(m!(ExpectedIdentifier))),
        }
    }
}

impl<'a> Parse<'a> for Identifier {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser
            .parse::<&'a Identifier>()
            .map(|identifier| identifier.clone())
    }
}
