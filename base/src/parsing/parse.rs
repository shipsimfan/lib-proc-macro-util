use crate::{Parser, Result};

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

impl<'a, T: Parse<'a>> Parse<'a> for Box<T> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        T::parse(parser).map(Box::new)
    }
}

impl<'a, T: Parse<'a>> Parse<'a> for Option<T> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<T>() {
            parser.parse().map(|value| Some(value))
        } else {
            Ok(None)
        }
    }
}
