use base::{tokens::Identifier, Generator, Parse, Parser, ToTokens};
use group::Group;
use list::TokenList;
use tree::TokenTree;

mod group;
mod list;
mod tree;

pub struct ToTokensMacro {
    generator_ident: Identifier,
    tokens: TokenList,
}

impl<'a> Parse<'a> for ToTokensMacro {
    fn parse(parser: &mut Parser<'a>) -> base::Result<Self> {
        Ok(ToTokensMacro {
            generator_ident: parser
                .parse()
                .map_err(|error| error.append("expected the generator's variable name"))?,
            tokens: parser.parse()?,
        })
    }
}

impl ToTokens for ToTokensMacro {
    fn to_tokens(&self, generator: &mut Generator) {
        let mut id = 0;
        self.tokens
            .to_tokens(generator, &self.generator_ident, &mut id);
    }
}
