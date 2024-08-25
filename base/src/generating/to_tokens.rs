use proc_macro::Span;

use crate::{tokens::Literal, Generator};

/// An object which can be converted to tokens
pub trait ToTokens {
    /// Inserts the tokens for this object into the [`Generator`]
    ///
    /// ## Parameters
    ///  * `generator` - The generator taking the tokens
    fn to_tokens(&self, generator: &mut Generator);
}

impl<T: ToTokens> ToTokens for Box<T> {
    fn to_tokens(&self, generator: &mut Generator) {
        self.as_ref().to_tokens(generator)
    }
}

impl<T: ToTokens> ToTokens for Option<T> {
    fn to_tokens(&self, generator: &mut Generator) {
        if let Some(t) = self {
            t.to_tokens(generator)
        }
    }
}

impl<T: ToTokens> ToTokens for &T {
    fn to_tokens(&self, generator: &mut Generator) {
        (*self).to_tokens(generator)
    }
}

impl<T: ToTokens> ToTokens for Vec<T> {
    fn to_tokens(&self, generator: &mut Generator) {
        for token in self {
            token.to_tokens(generator);
        }
    }
}

impl ToTokens for usize {
    fn to_tokens(&self, generator: &mut Generator) {
        Literal::new_usize_unsuffixed(*self, Span::call_site()).to_tokens(generator)
    }
}

impl ToTokens for str {
    fn to_tokens(&self, generator: &mut Generator) {
        Literal::new_string(self, Span::call_site()).to_tokens(generator)
    }
}

impl ToTokens for String {
    fn to_tokens(&self, generator: &mut Generator) {
        Literal::new_string(self, Span::call_site()).to_tokens(generator)
    }
}

impl ToTokens for [u8] {
    fn to_tokens(&self, generator: &mut Generator) {
        Literal::new_byte_string(self, Span::call_site()).to_tokens(generator)
    }
}
