use crate::{
    supported_languages::*,
    tokens::{Group, TokenTree},
    Parse, Parser, Result,
};
use i18n_translation::m;

i18n_translation::message_key!( ExpectedGroup [
    EN => { "expected a group" },
    FR => { "un groupe était attendu" },
    ZH => { "预期的组" },
]);

impl<'a> Parse<'a> for &'a Group {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        match parser.next() {
            Some(TokenTree::Group(group)) => Ok(group),
            _ => Err(parser.error(m!(ExpectedGroup))),
        }
    }
}

impl<'a> Parse<'a> for Group {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.parse::<&'a Group>().map(|group| group.clone())
    }
}
