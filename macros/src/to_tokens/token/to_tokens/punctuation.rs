use proc_macro_util_base::{
    tokens::{Identifier, Punctuation},
    Generator, Spacing, ToTokens, Token,
};

pub fn to_tokens(
    punctuation: &Punctuation,
    generator_name: &Identifier,
    generator: &mut Generator,
) {
    generate_to_tokens(punctuation.clone(), generator_name.clone(), generator);
}

fn generate_to_tokens(
    punctuation: Punctuation,
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
        punctuation,
        generator_name,
        &mut generator.group_parenthesis(),
    );
    Token![;]().to_tokens(generator);
}

fn generate_to_tokens_parameters(
    punctuation: Punctuation,
    generator_name: Identifier,
    generator: &mut Generator,
) {
    generate_new(punctuation, generator);
    Token![,]().to_tokens(generator);
    generator_name.to_tokens(generator);
}

fn generate_new(punctuation: Punctuation, generator: &mut Generator) {
    Token![::]().to_tokens(generator);
    Identifier::new("proc_macro_util").to_tokens(generator);
    Token![::]().to_tokens(generator);
    Identifier::new("tokens").to_tokens(generator);
    Token![::]().to_tokens(generator);
    Identifier::new("Punctuation").to_tokens(generator);
    Token![::]().to_tokens(generator);
    Identifier::new(match punctuation.spacing() {
        Spacing::Alone => "new",
        Spacing::Joint => "new_joint",
    })
    .to_tokens(generator);
    punctuation
        .as_char()
        .to_tokens(&mut generator.group_parenthesis());
}
