use crate::{Parse, Parser, Result};

impl<'a, T1: Parse<'a>, T2: Parse<'a>> Parse<'a> for (T1, T2) {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok((parser.parse()?, parser.parse()?))
    }
}

impl<'a, T1: Parse<'a>, T2: Parse<'a>, T3: Parse<'a>> Parse<'a> for (T1, T2, T3) {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok((parser.parse()?, parser.parse()?, parser.parse()?))
    }
}

impl<'a, T1: Parse<'a>, T2: Parse<'a>, T3: Parse<'a>, T4: Parse<'a>> Parse<'a>
    for (T1, T2, T3, T4)
{
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok((
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
        ))
    }
}

impl<'a, T1: Parse<'a>, T2: Parse<'a>, T3: Parse<'a>, T4: Parse<'a>, T5: Parse<'a>> Parse<'a>
    for (T1, T2, T3, T4, T5)
{
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok((
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
        ))
    }
}

impl<
        'a,
        T1: Parse<'a>,
        T2: Parse<'a>,
        T3: Parse<'a>,
        T4: Parse<'a>,
        T5: Parse<'a>,
        T6: Parse<'a>,
    > Parse<'a> for (T1, T2, T3, T4, T5, T6)
{
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok((
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
        ))
    }
}

impl<
        'a,
        T1: Parse<'a>,
        T2: Parse<'a>,
        T3: Parse<'a>,
        T4: Parse<'a>,
        T5: Parse<'a>,
        T6: Parse<'a>,
        T7: Parse<'a>,
    > Parse<'a> for (T1, T2, T3, T4, T5, T6, T7)
{
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok((
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
        ))
    }
}

impl<
        'a,
        T1: Parse<'a>,
        T2: Parse<'a>,
        T3: Parse<'a>,
        T4: Parse<'a>,
        T5: Parse<'a>,
        T6: Parse<'a>,
        T7: Parse<'a>,
        T8: Parse<'a>,
    > Parse<'a> for (T1, T2, T3, T4, T5, T6, T7, T8)
{
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok((
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
            parser.parse()?,
        ))
    }
}
