use super::TokenTree;
use proc_macro_util_base::{tokens::Identifier, Generator, Parse, Parser, Result};

pub(super) struct TokenList {
    tokens: Vec<TokenTree>,
}

impl TokenList {
    pub(super) fn to_tokens(
        self,
        generator: &mut Generator,
        generator_ident: &Identifier,
        generator_ref: bool,
        id: &mut usize,
    ) {
        for token in self.tokens {
            token.to_tokens(generator, generator_ident, generator_ref, id);
        }
    }
}

impl<'a> Parse<'a> for TokenList {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut tokens = Vec::new();

        while !parser.empty() {
            tokens.push(parser.parse()?);
        }

        Ok(TokenList { tokens })
    }
}
