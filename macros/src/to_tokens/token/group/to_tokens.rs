use super::Group;
use proc_macro_util_base::{tokens::Identifier, Delimiter, Generator, ToTokens, Token};

impl<'a> Group<'a> {
    pub fn to_tokens(self, generator_name: &Identifier, generator: &mut Generator) {
        if self.tokens.tokens.len() == 0 {
            generate_empty(self.delimiter, generator_name.clone(), generator)
        } else {
            generate_body(
                self.delimiter,
                self.tokens,
                generator_name.clone(),
                &mut generator.group_brace(),
            )
        }
    }
}

fn generate_empty(delimiter: Delimiter, generator_name: Identifier, generator: &mut Generator) {
    generator_name.to_tokens(generator);
    Token![.]().to_tokens(generator);
    Identifier::new(match delimiter {
        Delimiter::Brace => "group_brace",
        Delimiter::Bracket => "group_bracket",
        Delimiter::Parenthesis => "group_parenthesis",
        Delimiter::None => unimplemented!(),
    })
    .to_tokens(generator);
    generator.group_parenthesis();
    Token![;]().to_tokens(generator);
}

fn generate_body(
    delimiter: Delimiter,
    to_tokens: crate::to_tokens::ToTokens,
    generator_name: Identifier,
    generator: &mut Generator,
) {
    Token![let]().to_tokens(generator);
    Token![mut]().to_tokens(generator);
    generator_name.clone().to_tokens(generator);
    Token![=]().to_tokens(generator);
    generate_empty(delimiter, generator_name, generator);

    to_tokens.to_tokens(generator);
}
