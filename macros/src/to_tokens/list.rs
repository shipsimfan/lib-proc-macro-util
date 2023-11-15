use super::TokenTree;
use base::{tokens::Identifier, Generator, Parse, Parser, Result};

pub(super) struct TokenList {
    tokens: Vec<TokenTree>,
}

impl TokenList {
    pub(super) fn to_tokens(
        &self,
        generator: &mut Generator,
        generator_ident: &Identifier,
        id: &mut usize,
    ) {
        for token in &self.tokens {
            token.to_tokens(generator, generator_ident, id);
        }
    }
}

impl Parse for TokenList {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let mut tokens = Vec::new();

        while !parser.empty() {
            tokens.push(parser.parse()?);
        }

        Ok(TokenList { tokens })
    }
}
