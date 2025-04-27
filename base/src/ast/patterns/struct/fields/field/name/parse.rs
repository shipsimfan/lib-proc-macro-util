use crate::{ast::patterns::StructPatternFieldName, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for StructPatternFieldName<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![ref]>() || parser.peek::<Token![mut]>() {
            return Ok(StructPatternFieldName::Identifier(
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
            ));
        }

        if let Ok(index) = parser.step_parse() {
            return Ok(StructPatternFieldName::Index(
                index,
                parser.parse()?,
                parser.parse()?,
            ));
        }

        let identifier = parser.parse()?;

        if let Ok(colon) = parser.step_parse() {
            return Ok(StructPatternFieldName::IdentifierPattern(
                identifier,
                colon,
                parser.parse()?,
            ));
        }

        Ok(StructPatternFieldName::Identifier(None, None, identifier))
    }
}
