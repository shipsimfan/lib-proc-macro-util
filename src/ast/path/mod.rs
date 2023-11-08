use crate::{
    ast::Punctuated,
    parsing::{Parse, Parser},
    tokens::{DoubleColon, Ident, Parenthesis},
    Result, Token,
};

mod arguments;
mod segment;

pub use arguments::PathArguments;
pub use segment::PathSegment;

#[derive(Clone)]
pub struct Path {
    pub leading_colon: Option<DoubleColon>,
    pub segments: Punctuated<PathSegment, DoubleColon>,
}

impl Path {
    pub fn parse_mod_style(parser: &mut Parser) -> Result<Self> {
        let leading_colon = parser.parse()?;

        let mut segments = Punctuated::new();
        loop {
            if !parser.peek::<Ident>()
                && !parser.peek::<Token![super]>()
                && !parser.peek::<Token![self]>()
                && !parser.peek::<Token![Self]>()
                && !parser.peek::<Token![crate]>()
            {
                break;
            }

            let ident = Ident::parse(parser)?;
            segments.push_value(PathSegment::from(ident));

            if !parser.peek::<Token![::]>() {
                break;
            }

            let punctuation = parser.parse()?;
            segments.push_punctuation(punctuation);
        }

        if segments.empty() {
            return Err(parser.parse::<Ident>().unwrap_err());
        } else if segments.trailing_punct() {
            return Err(parser.error("expected path segment after `::`"));
        }

        Ok(Path {
            leading_colon,
            segments,
        })
    }
}

impl<'a> Parse<'a> for Path {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let leading_colon = parser.parse()?;
        let segments = {
            let mut segments = Punctuated::new();
            let mut value = parser.parse()?;
            segments.push_value(value);
            segments
        };

        let mut path = Path {
            leading_colon,
            segments,
        };

        while parser.peek::<Token![::]>() && !parser.peek3::<Parenthesis>() {
            path.segments.push_punctuation(parser.parse()?);
            path.segments.push_value(parser.parse()?);
        }

        Ok(path)
    }
}
