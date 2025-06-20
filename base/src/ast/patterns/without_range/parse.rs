use crate::{
    ast::{
        expressions::PathExpression,
        patterns::{
            GroupedPattern, PathPattern, ReferencePattern, SlicePattern, StructPattern,
            TuplePattern, TuplePatternItems, TupleStructPattern,
        },
        PatternWithoutRange,
    },
    tokens::Group,
    Delimiter, Parse, Parser, Result, Token,
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
                return Err(group.span.start().error("expected `[` or `(`"));
            };

            if !parser.empty() {
                return Err(parser.error("unexpected token"));
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

        if let Ok(path) = parser.step_parse() {
            if let Ok(group) = parser.step(|parser| {
                let group: &'a Group = parser.parse()?;
                match group.delimiter {
                    Delimiter::Brace | Delimiter::Bracket => Ok(group),
                    _ => Err(group.span.start().error("expected `{` or `[`")),
                }
            }) {
                let mut parser = group.parser();
                let pattern = match group.delimiter {
                    Delimiter::Brace => PatternWithoutRange::Struct(StructPattern {
                        path,
                        elements: parser.parse()?,
                    }),
                    Delimiter::Parenthesis => {
                        PatternWithoutRange::TupleStruct(TupleStructPattern {
                            path,
                            items: parser.parse()?,
                        })
                    }
                    _ => return Err(group.span.start().error("expected `(` or `{`")),
                };

                if !parser.empty() {
                    return Err(parser.error("unexpected token"));
                }

                return Ok(pattern);
            }

            return Ok(PatternWithoutRange::Path(PathPattern {
                path: PathExpression::Normal(path),
            }));
        }

        if let Ok(path) = parser.step_parse() {
            return Ok(PatternWithoutRange::Path(PathPattern {
                path: PathExpression::QualifiedPathInExpression(path),
            }));
        }

        if let Ok(macro_invocation) = parser.step_parse() {
            return Ok(PatternWithoutRange::MacroInvocation(macro_invocation));
        }

        if let Ok(identifier) = parser.step_parse() {
            return Ok(PatternWithoutRange::Identifier(identifier));
        }

        Err(parser.error("expected a pattern without range"))
    }
}
