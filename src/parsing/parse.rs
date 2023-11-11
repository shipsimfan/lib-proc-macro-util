use crate::{Parser, Result};

/// An object that can be created by parsing tokens
pub trait Parse: Sized {
    /// Creates this object from parsing tokens
    ///
    /// ## Parameters
    ///  * `parser` - The parser which holds the token stream
    ///
    /// ## Return Value
    /// Returns the newly created object on success.
    fn parse(parser: &mut Parser) -> Result<Self>;
}
