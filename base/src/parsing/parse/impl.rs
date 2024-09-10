use crate::{Parse, Parser, Result};

impl<'a, T: Parse<'a>> Parse<'a> for Vec<T> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut values = Vec::new();
        while let Ok(value) = parser.step(Parser::parse) {
            values.push(value);
        }
        Ok(values)
    }
}

impl<'a, T: Parse<'a>> Parse<'a> for Box<T> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        T::parse(parser).map(Box::new)
    }
}

impl<'a, T: Parse<'a>> Parse<'a> for Option<T> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(parser.step(Parser::parse).ok())
    }
}
