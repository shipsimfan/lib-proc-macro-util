use crate::{
    ast::{GenericArgument, Punctuated},
    parsing::{Parse, Parser},
    tokens::{Comma, DoubleColon, LeftTriangle, RightTriangle},
    Result, Token,
};

#[derive(Clone)]
pub struct AngleBracketGenerics {
    pub double_colon: Option<DoubleColon>,
    pub left_triangle: LeftTriangle,
    pub args: Punctuated<GenericArgument, Comma>,
    pub right_triangle: RightTriangle,
}

impl<'a> Parse<'a> for AngleBracketGenerics {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let double_colon = parser.parse()?;
        let left_triangle = parser.parse()?;

        let mut args = Punctuated::new();
        loop {
            if parser.peek::<Token![>]>() {
                break;
            }

            args.push_value(parser.parse()?);

            if parser.peek::<Token![>]>() {
                break;
            }

            args.push_punctuation(parser.parse()?);
        }

        let right_triangle = parser.parse()?;

        Ok(AngleBracketGenerics {
            double_colon,
            left_triangle,
            args,
            right_triangle,
        })
    }
}
