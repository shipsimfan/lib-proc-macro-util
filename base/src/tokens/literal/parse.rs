use crate::{
    supported_languages::*,
    tokens::{Literal, TokenTree},
    Parse, Parser, Result,
};
use i18n_translation::m;

i18n_translation::message_key!( ExpectedLiteral [
    EN => { "expected a literal" },
    FR => { "une valeur littérale était attendue" },
    ZH => { "预期的字面值" },
]);

impl<'a> Parse<'a> for &'a Literal {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        match parser.next() {
            Some(TokenTree::Literal(literal)) => Ok(literal.into()),
            _ => Err(parser.error(m!(ExpectedLiteral))),
        }
    }
}

impl<'a> Parse<'a> for Literal {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.parse::<&'a Literal>().map(|literal| literal.clone())
    }
}
