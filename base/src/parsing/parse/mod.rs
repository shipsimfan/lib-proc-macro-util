use crate::{Parser, Result};

mod r#impl;

/// An object that can be created by parsing tokens
pub trait Parse<'a>: Sized {
    /// Creates this object from parsing tokens
    ///
    /// ## Parameters
    ///  * `parser` - The parser which holds the token stream
    ///
    /// ## Return Value
    /// Returns the newly created object on success.
    fn parse(parser: &mut Parser<'a>) -> Result<Self>;
}
