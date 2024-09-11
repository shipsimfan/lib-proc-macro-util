mod generator;
mod into_token_stream;
mod to_tokens;

pub use generator::Generator;
pub use into_token_stream::into_token_stream;
pub use to_tokens::ToTokens;

/// Generates a [`TokenStream`] for a given type
pub fn generate<T: ToTokens + ?Sized>(value: &T) -> proc_macro::TokenStream {
    let mut tokens = Vec::new();
    let mut generator = Generator::new(&mut tokens);
    generator.generate(value);

    into_token_stream(tokens)
}
