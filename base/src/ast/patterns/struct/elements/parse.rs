use crate::{
    ast::patterns::{
        StructPatternElements, StructPatternEtCetera, StructPatternField, StructPatternFields,
    },
    Error, Parse, Parser, Result, Span,
};

impl<'a> Parse<'a> for StructPatternElements<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let attributes = parser.parse()?;
        if let Ok(dots) = parser.step_parse() {
            return Ok(StructPatternElements::EtCetera(StructPatternEtCetera {
                attributes,
                dots,
            }));
        }

        let first = StructPatternField {
            attributes,
            name: parser.parse()?,
        };

        let mut remaining = Vec::new();
        let final_comma = loop {
            let comma = match parser.step_parse() {
                Ok(comma) => comma,
                Err(_) => break None,
            };

            let attributes = parser.parse()?;

            if let Ok(dots) = parser.step_parse() {
                return Ok(StructPatternElements::Fields(
                    StructPatternFields { first, remaining },
                    Some((comma, Some(StructPatternEtCetera { attributes, dots }))),
                ));
            }

            if let Ok(name) = parser.step_parse() {
                remaining.push((comma, StructPatternField { attributes, name }));
                continue;
            }

            if attributes.len() > 0 {
                return Err(Error::new_at("unexpected token", Span::call_site()));
            }

            break Some(comma);
        };

        Ok(StructPatternElements::Fields(
            StructPatternFields { first, remaining },
            final_comma.map(|comma| (comma, None)),
        ))
    }
}
