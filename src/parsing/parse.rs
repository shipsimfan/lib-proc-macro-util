use super::Parser;
use crate::Result;

pub trait Parse<'a>: Sized {
    fn parse(parser: &mut Parser<'a>) -> Result<Self>;
}
