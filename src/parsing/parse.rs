use super::Parser;
use crate::Result;

pub trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Result<Self>;
}
