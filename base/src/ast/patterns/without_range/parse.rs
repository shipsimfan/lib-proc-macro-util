use crate::{
    ast::{
        patterns::{GroupedPattern, ReferencePattern, SlicePattern},
        PatternWithoutRange,
    },
    tokens::Group,
    Delimiter, Error, Parse, Parser, Result, Token,
};

impl<'a> Parse<'a> for PatternWithoutRange<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![&]>() || parser.peek::<Token![&&]>() {
            return ReferencePattern::parse(parser).map(PatternWithoutRange::Reference);
        }

        if let Ok(group) = parser.step_parse::<&'a Group>() {
            let mut parser = group.parser();
            let pattern = if group.delimiter == Delimiter::Bracket {
                PatternWithoutRange::Slice(SlicePattern {
                    items: parser.parse()?,
                })
            } else if group.delimiter == Delimiter::Parenthesis {
                PatternWithoutRange::Grouped(GroupedPattern {
                    pattern: parser.parse()?,
                })
            } else {
                return Err(Error::new_at("unexpected token", group.span));
            };

            if !parser.empty() {
                return Err(Error::new_at("unexpected token", group.span));
            }

            return Ok(pattern);
        }

        if let Ok(wildcard) = parser.step_parse() {
            return Ok(PatternWithoutRange::Wildcard(wildcard));
        }

        if let Ok(rest) = parser.step_parse() {
            return Ok(PatternWithoutRange::Rest(rest));
        }

        if let Ok(literal) = parser.step_parse() {
            return Ok(PatternWithoutRange::Literal(literal));
        }

        if let Ok(identifier) = parser.step_parse() {
            return Ok(PatternWithoutRange::Identifier(identifier));
        }

        if let Ok(path) = parser.step_parse() {
            return Ok(PatternWithoutRange::Path(path));
        }

        if let Ok(macro_invocation) = parser.step_parse() {
            return Ok(PatternWithoutRange::MacroInvocation(macro_invocation));
        }

        Err(Error::new_at(
            "expected a pattern without range",
            parser.span(),
        ))
    }
}
