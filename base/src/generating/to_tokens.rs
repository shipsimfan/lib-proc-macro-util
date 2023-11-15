use crate::Generator;

/// An object which can be converted to tokens
pub trait ToTokens {
    /// Inserts the tokens for this object into the [`Generator`]
    ///
    /// ## Parameters
    ///  * `generator` - The generator taking the tokens
    fn to_tokens(&self, generator: &mut Generator);
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
