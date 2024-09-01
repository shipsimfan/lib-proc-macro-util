use crate::parsing::TokenBuffer;

// rustdoc imports
#[allow(unused_imports)]
use proc_macro::TokenStream;

mod generator;
mod to_tokens;

pub use generator::Generator;
pub use to_tokens::ToTokens;

/// Generates a [`TokenStream`] for a given type
pub fn generate<T: ToTokens + ?Sized>(value: &T) -> proc_macro::TokenStream {
    let mut token_buffer = TokenBuffer::new();
    let mut generator = Generator::new(&mut token_buffer);
    generator.generate(value);
    token_buffer.into()
}
