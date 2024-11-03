use proc_macro_util_base::{tokens::Identifier, Generator, ToTokens, Token};

pub fn to_tokens(identifier: &Identifier, generator_name: &Identifier, generator: &mut Generator) {
    generate_to_tokens(identifier.clone(), generator_name.clone(), generator);
}

fn generate_to_tokens(
    identifier: Identifier,
    generator_name: Identifier,
    generator: &mut Generator,
) {
    Token![::]().to_tokens(generator);
    Identifier::new("proc_macro_util").to_tokens(generator);
    Token![::]().to_tokens(generator);
    Identifier::new("ToTokens").to_tokens(generator);
    Token![::]().to_tokens(generator);
    Identifier::new("to_tokens").to_tokens(generator);
    generate_to_tokens_parameters(
        identifier,
        generator_name,
        &mut generator.group_parenthesis(),
    );
    Token![;]().to_tokens(generator);
}

fn generate_to_tokens_parameters(
    identifier: Identifier,
    generator_name: Identifier,
    generator: &mut Generator,
) {
    generate_new(identifier, generator);
    Token![,]().to_tokens(generator);
    generator_name.to_tokens(generator);
}

fn generate_new(identifier: Identifier, generator: &mut Generator) {
    Token![::]().to_tokens(generator);
    Identifier::new("proc_macro_util").to_tokens(generator);
    Token![::]().to_tokens(generator);
    Identifier::new("tokens").to_tokens(generator);
    Token![::]().to_tokens(generator);
    Identifier::new("Identifier").to_tokens(generator);
    Token![::]().to_tokens(generator);
    Identifier::new("new").to_tokens(generator);
    generate_new_parameters(identifier, &mut generator.group_parenthesis());
}

fn generate_new_parameters(identifier: Identifier, generator: &mut Generator) {
    Token![::]().to_tokens(generator);
    Identifier::new("std").to_tokens(generator);
    Token![::]().to_tokens(generator);
    Identifier::new("stringify").to_tokens(generator);
    Token![!]().to_tokens(generator);
    identifier.to_tokens(&mut generator.group_parenthesis());
}
