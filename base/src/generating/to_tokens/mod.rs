use crate::Generator;

mod r#box;
mod number;
mod option;
mod slice;
mod string;

/// An object which can be converted to tokens
pub trait ToTokens {
    /// Inserts the tokens for this object into the [`Generator`]
    ///
    /// ## Parameters
    ///  * `generator` - The generator taking the tokens
    fn to_tokens(self, generator: &mut Generator);
}
