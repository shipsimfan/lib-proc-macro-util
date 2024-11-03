use crate::{supported_languages::*, tokens::TokenTree, Parse, Parser, Result};
use i18n::translation::m;

i18n::translation::message_key!( EXPECTED_TOKEN_TREE [
    EN => { "expected a token tree" },
    FR => { "un arbre de jetons était attendu" },
    ZH => { "预期的标记树" },
]);

impl<'a> Parse<'a> for &'a TokenTree {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.next().ok_or(parser.error(m!(EXPECTED_TOKEN_TREE)))
    }
}

impl<'a> Parse<'a> for TokenTree {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser
            .parse::<&'a TokenTree>()
            .map(|token_tree| token_tree.clone())
    }
}
