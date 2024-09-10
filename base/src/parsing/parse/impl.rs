use crate::{Parse, Parser, Result};

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
