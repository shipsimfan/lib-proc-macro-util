use crate::{
    ast::{
        patterns::{
            GroupedPattern, ReferencePattern, SlicePattern, TuplePattern, TuplePatternItems,
        },
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
                if parser.empty() {
                    PatternWithoutRange::Tuple(TuplePattern { items: None })
                } else if let Ok(rest) = parser.step_parse() {
                    PatternWithoutRange::Tuple(TuplePattern {
                        items: Some(TuplePatternItems::Rest(rest)),
                    })
                } else {
                    let pattern = parser.parse()?;

                    if let Ok(comma) = parser.step_parse() {
                        PatternWithoutRange::Tuple(TuplePattern {
                            items: Some(if let Ok(second) = parser.step_parse() {
                                TuplePatternItems::Multiple {
                                    first: pattern,
                                    first_comma: comma,
                                    second,
                                    remaining: parser.parse()?,
                                    last: parser.parse()?,
                                }
                            } else {
                                TuplePatternItems::Single(pattern, comma)
                            }),
                        })
                    } else {
                        PatternWithoutRange::Grouped(GroupedPattern { pattern })
                    }
                }
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

        if let Ok(tuple_struct) = parser.step_parse() {
            return Ok(PatternWithoutRange::TupleStruct(tuple_struct));
        }

        if let Ok(path) = parser.step_parse() {
            return Ok(PatternWithoutRange::Path(path));
        }

        if let Ok(macro_invocation) = parser.step_parse() {
            return Ok(PatternWithoutRange::MacroInvocation(macro_invocation));
        }

        if let Ok(identifier) = parser.step_parse() {
            return Ok(PatternWithoutRange::Identifier(identifier));
        }

        Err(Error::new_at(
            "expected a pattern without range",
            parser.span(),
        ))
    }
}
