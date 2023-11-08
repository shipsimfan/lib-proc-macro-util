use crate::{
    ast::{Expression, ExpressionLiteral, Path},
    parsing::{Parse, Parser},
    tokens::{Brace, Bracket, Literal, Parenthesis},
    Result, Token,
};

mod list;
mod name_value;

pub use list::MetaList;
pub use name_value::MetaNameValue;

#[derive(Clone)]
pub enum Meta {
    Path(Path),
    List(MetaList),
    NameValue(MetaNameValue),
}

impl<'a> Parse<'a> for Meta {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let path: Path = parser.parse()?;

        if parser.peek::<Parenthesis>() || parser.peek::<Bracket>() || parser.peek::<Brace>() {
            let (delimiter, tokens) = parser
                .delimiter()
                .ok_or_else(|| parser.error("expected delimiter"))?;
            Ok(Meta::List(MetaList {
                path,
                delimiter,
                tokens: tokens.into(),
            }))
        } else if parser.peek::<Token![=]>() {
            let equals = parser.parse()?;
            let mut ahead = parser.clone();
            let literal: Option<Literal> = ahead.parse()?;
            let value = if let (Some(literal), true) = (literal, ahead.is_empty()) {
                *parser = ahead;
                Expression::Literal(ExpressionLiteral {
                    attributes: Vec::new(),
                    literal,
                })
            } else if parser.peek::<Token![#]>() && parser.peek2::<Bracket>() {
                return Err(parser.error("unexected attribute inside of attribute"));
            } else {
                parser.parse()?
            };

            Ok(Meta::NameValue(MetaNameValue {
                path,
                equals,
                value,
            }))
        } else {
            Ok(Meta::Path(path))
        }
    }
}
