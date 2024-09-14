use super::{Group, Token};
use proc_macro_util_base::{
    tokens::{Identifier, Punctuation, TokenTree},
    Parser, Result,
};

/// Attempts to parse `punctuation` into either a [`Token::Normal`] or a [`Token::Variable`]
fn parse_punctuation<'a>(punctuation: &'a Punctuation, parser: &mut Parser<'a>) -> Token<'a> {
    if punctuation.as_char() != '#' || !parser.peek::<Identifier>() {
        return Token::Punctuation(punctuation);
    }

    Token::Variable(parser.parse().unwrap())
}

impl<'a> Token<'a> {
    pub fn parse(parser: &mut Parser<'a>, generator: &'a Identifier) -> Result<Self> {
        let token_tree = parser
            .next()
            .ok_or_else(|| parser.error("expected a token"))?;

        Ok(match token_tree {
            TokenTree::Group(group) => Token::Group(Group::parse(group, generator)?),
            TokenTree::Identifier(identifier) => Token::Identifier(identifier),
            TokenTree::Literal(literal) => Token::Literal(literal),
            TokenTree::Punctuation(punctuation) => parse_punctuation(punctuation, parser),
        })
    }
}
