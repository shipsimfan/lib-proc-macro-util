use super::Parser;
use crate::{
    tokens::{Ident, Literal},
    Result,
};

pub trait Parse<'a>: Sized {
    fn parse(parser: &mut Parser<'a>) -> Result<Self>;
}

impl<'a> Parse<'a> for Ident {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.ident().ok_or(parser.error("expected an identifier"))
    }
}

impl<'a> Parse<'a> for Literal {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser
            .literal()
            .ok_or(parser.error("expected an identifier"))
    }
}

impl<'a, T: Parse<'a>> Parse<'a> for Option<T> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<T>() {
            Ok(Some(parser.parse()?))
        } else {
            Ok(None)
        }
    }
}
