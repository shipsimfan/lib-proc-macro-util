mod generator;
mod to_tokens;

pub use generator::Generator;
pub use to_tokens::ToTokens;

/// Generates a [`TokenStream`] for a given type
pub fn generate<T: ToTokens + ?Sized>(value: &T) -> proc_macro::TokenStream {
    let mut tokens = Vec::new();
    let mut generator = Generator::new(&mut tokens);
    generator.generate(value);

    let mut token_stream = proc_macro::TokenStream::new();
    token_stream.extend(tokens.into_iter().map(|token| token.into()));
    token_stream
}
