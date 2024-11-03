use proc_macro_util_base::{
    tokens::{Identifier, Literal},
    Generator, ToTokens, Token,
};

pub fn to_tokens(literal: &Literal, generator_name: &Identifier, generator: &mut Generator) {
    generate_to_tokens(literal.clone(), generator_name.clone(), generator);
}

fn generate_to_tokens(literal: Literal, generator_name: Identifier, generator: &mut Generator) {
    Token![::]().to_tokens(generator);
    Identifier::new("proc_macro_util").to_tokens(generator);
    Token![::]().to_tokens(generator);
    Identifier::new("ToTokens").to_tokens(generator);
    Token![::]().to_tokens(generator);
    Identifier::new("to_tokens").to_tokens(generator);
    generate_to_tokens_parameters(literal, generator_name, &mut generator.group_parenthesis());
    Token![;]().to_tokens(generator);
}

fn generate_to_tokens_parameters(
    literal: Literal,
    generator_name: Identifier,
    generator: &mut Generator,
) {
    generate_new(literal, generator);
    Token![,]().to_tokens(generator);
    generator_name.to_tokens(generator);
}

fn generate_new(literal: Literal, generator: &mut Generator) {
    Token![::]().to_tokens(generator);
    Identifier::new("proc_macro_util").to_tokens(generator);
    Token![::]().to_tokens(generator);
    Identifier::new("tokens").to_tokens(generator);
    Token![::]().to_tokens(generator);
    Identifier::new("Literal").to_tokens(generator);
    Token![::]().to_tokens(generator);
    Identifier::new("new").to_tokens(generator);
    literal.to_tokens(&mut generator.group_parenthesis());
}
