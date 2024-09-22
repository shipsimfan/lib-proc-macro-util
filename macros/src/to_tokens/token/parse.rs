use super::{Group, Token};
use i18n::m;
use proc_macro_util_base::{
    supported_languages::*,
    tokens::{Identifier, Punctuation, TokenTree},
    Parser, Result,
};

i18n::message_key!( EXPECTED_TOKEN [
    EN => { "expected a token" },
    FR => { "un jeton était attendu" },
    ZH => { "预期的标记" },
]);

/// Attempts to parse `punctuation` into either a [`Token::Punctuation`] or a [`Token::Variable`]
fn parse_punctuation<'a>(punctuation: &'a Punctuation, parser: &mut Parser<'a>) -> Token<'a> {
    if punctuation.as_char() == '#' {
        if let Ok(identifier) = parser.step(Parser::parse) {
            return Token::Variable(identifier);
        }
    }

    return Token::Punctuation(punctuation);
}

impl<'a> Token<'a> {
    pub fn parse(parser: &mut Parser<'a>, generator: &'a Identifier) -> Result<Self> {
        let token_tree = parser
            .next()
            .ok_or_else(|| parser.error(m!(EXPECTED_TOKEN)))?;

        Ok(match token_tree {
            TokenTree::Group(group) => Token::Group(Group::parse(group, generator)?),
            TokenTree::Identifier(identifier) => Token::Identifier(identifier),
            TokenTree::Literal(literal) => Token::Literal(literal),
            TokenTree::Punctuation(punctuation) => parse_punctuation(punctuation, parser),
        })
    }
}
