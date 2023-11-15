use super::Group;
use base::{
    tokens::{Identifier, Literal, Punctuation, TokenTree as RawTokenTree},
    Generator, Parse, Parser, Result, ToTokens, Token,
};
use proc_macro::Delimiter;

pub(super) enum TokenTree {
    Group(Group),
    Identifier(Identifier),
    Literal(Literal),
    Punctuation(Punctuation),
    Variable(Identifier),
}

fn generator_dot(generator: &mut Generator, ident: &Identifier) {
    generator.identifier(ident.clone());
    Token![.].to_tokens(generator);
}

impl TokenTree {
    pub(super) fn to_tokens(
        &self,
        generator: &mut Generator,
        generator_ident: &Identifier,
        id: &mut usize,
    ) {
        match self {
            TokenTree::Group(group) => return group.to_tokens(generator, generator_ident, id),
            TokenTree::Identifier(identifier) => {
                generator_dot(generator, generator_ident);
                generator.identifier_string("identifier_string");

                let mut parameters = generator.group(Delimiter::Parenthesis);
                parameters.literal_string(&identifier.to_string());
            }
            TokenTree::Literal(literal) => {
                generator_dot(generator, generator_ident);
                generator.identifier_string("generate");

                let mut parameters = generator.group(Delimiter::Parenthesis);
                Token![&].to_tokens(&mut parameters);
                literal.to_tokens(&mut parameters);
            }
            TokenTree::Punctuation(punctuation) => {
                generator.identifier_string("Token");
                Token![!].to_tokens(generator);

                let mut token_parameters = generator.group(Delimiter::Bracket);
                token_parameters.punctuation(punctuation.clone());

                Token![.].to_tokens(generator);
                generator.identifier_string("to_tokens");

                let mut parameters = generator.group(Delimiter::Parenthesis);
                parameters.identifier(generator_ident.clone());
            }
            TokenTree::Variable(variable) => {
                generator_dot(generator, generator_ident);
                generator.identifier_string("to_tokens");

                let mut parameters = generator.group(Delimiter::Parenthesis);
                Token![&].to_tokens(&mut parameters);
                parameters.identifier(variable.clone());
            }
        }
        Token![;].to_tokens(generator);
    }
}

impl Parse for TokenTree {
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(
            match parser.next().ok_or(parser.error("expected a token tree"))? {
                RawTokenTree::Group(group) => {
                    TokenTree::Group(Group::new(group.delimiter(), group.tokens())?)
                }
                RawTokenTree::Identifier(identifier) => TokenTree::Identifier(identifier),
                RawTokenTree::Literal(literal) => TokenTree::Literal(literal),
                RawTokenTree::Punctuation(punctuation) => {
                    if punctuation.as_char() != '#' {
                        return Ok(TokenTree::Punctuation(punctuation));
                    }

                    match parser.identifier() {
                        Some(identifier) => TokenTree::Variable(identifier),
                        None => TokenTree::Punctuation(punctuation),
                    }
                }
            },
        )
    }
}