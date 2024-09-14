use super::TokenList;
use proc_macro_util_base::{
    tokens::Identifier, Delimiter, Generator, Parser, Result, Span, ToTokens, Token,
};

pub(super) struct Group {
    delimiter: Delimiter,
    tokens: TokenList,
}

const BASE_GENERATOR_NAME: &str = "__proc_macro_util_generator";

impl Group {
    pub(super) fn to_tokens(
        &self,
        generator: &mut Generator,
        generator_ident: &Identifier,
        id: &mut usize,
    ) {
        let new_generator_ident = Identifier::new(&format!("{BASE_GENERATOR_NAME}{id}"));
        *id += 1;

        Token![let]().to_tokens(generator);
        Token![mut]().to_tokens(generator);
        new_generator_ident.to_tokens(generator);
        Token![=]().to_tokens(generator);
        generator_ident.to_tokens(generator);
        Token![.]().to_tokens(generator);
        "group".to_tokens(generator);

        let mut parameters = generator.group(Delimiter::Parenthesis);
        Token![::]().to_tokens(&mut parameters);
        parameters.identifier_string("proc_macro_util");
        Token![::]().to_tokens(&mut parameters);
        parameters.identifier_string("Delimiter");
        Token![::]().to_tokens(&mut parameters);
        parameters.identifier_string(match self.delimiter {
            Delimiter::Brace => "Brace",
            Delimiter::Bracket => "Bracket",
            Delimiter::None => "None",
            Delimiter::Parenthesis => "Parenthesis",
        });

        Token![;]().to_tokens(generator);

        self.tokens
            .to_tokens(generator, &new_generator_ident, true, id);
    }
}

impl Group {
    pub(super) fn new(delimiter: Delimiter, mut parser: Parser) -> Result<Self> {
        let tokens = parser.parse()?;

        Ok(Group { delimiter, tokens })
    }
}
