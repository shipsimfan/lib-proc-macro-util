use crate::{Parse, Parser, Result};
use std::borrow::Cow;

impl<'a, T: Parse<'a>> Parse<'a> for Vec<T> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut values = Vec::new();
        while let Ok(value) = parser.step_parse() {
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
        Ok(parser.step_parse().ok())
    }
}

impl<'a, T: Clone> Parse<'a> for Cow<'a, T>
where
    &'a T: Parse<'a>,
{
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.parse().map(|value| Cow::Borrowed(value))
    }
}
