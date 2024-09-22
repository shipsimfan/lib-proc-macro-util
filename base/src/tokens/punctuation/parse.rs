use crate::{
    supported_languages::*,
    tokens::{Punctuation, TokenTree},
    Parse, Parser, Result,
};
use i18n::m;

i18n::message_key!( EXPECTED_PUNCTUATION [
    EN => { "expected punctuation" },
    FR => { "une ponctuation était attendue" },
    ZH => { "预期的标点符号" },
]);

impl<'a> Parse<'a> for &'a Punctuation {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        match parser.next() {
            Some(TokenTree::Punctuation(punctuation)) => Ok(punctuation.into()),
            _ => Err(parser.error(m!(EXPECTED_PUNCTUATION))),
        }
    }
}

impl<'a> Parse<'a> for Punctuation {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser
            .parse::<&'a Punctuation>()
            .map(|punctuation| punctuation.clone())
    }
}
