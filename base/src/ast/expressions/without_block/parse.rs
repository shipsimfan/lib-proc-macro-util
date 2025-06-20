use crate::{
    ast::{
        expressions::{
            ArithmeticOrLogicalExpression, AssignmentExpression, AwaitExpression, CallExpression,
            ComparisonExpression, CompoundAssignmentExpression, ErrorPropagationExpression,
            FieldExpression, IndexExpression, LazyBooleanExpression, MethodCallExpression,
            OperatorExpression, RangeExpression, TupleIndexExpression, TypeCastExpression,
        },
        ExpressionWithoutBlock, ExpressionWithoutBlockKind,
    },
    tokens::Group,
    Delimiter, Parse, Parser, Result,
};

impl<'a> Parse<'a> for ExpressionWithoutBlock<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Self::do_parse(parser, true)
    }
}

impl<'a> ExpressionWithoutBlock<'a> {
    /// Parse an [`ExpressionWithoutBlock`] without parsing a
    /// [`StructExpression`](crate::ast::expressions::StructExpression)
    pub fn parse_without_struct(parser: &mut Parser<'a>) -> Result<Self> {
        Self::do_parse(parser, false)
    }

    pub(crate) fn do_parse(parser: &mut Parser<'a>, include_struct: bool) -> Result<Self> {
        let mut ret = ExpressionWithoutBlock {
            attributes: parser.parse()?,
            kind: ExpressionWithoutBlockKind::do_parse(parser, include_struct)?,
        };

        loop {
            if let Ok(operator) = parser.step_parse() {
                let mut attributes = Vec::new();
                std::mem::swap(&mut attributes, &mut ret.attributes);

                return Ok(ExpressionWithoutBlock {
                    attributes,
                    kind: ExpressionWithoutBlockKind::Range(RangeExpression {
                        lower: Some(Box::new(ret.into_expression())),
                        operator,
                        upper: parser.parse()?,
                    }),
                });
            }

            if let Ok(dot) = parser.step_parse() {
                let mut attributes = Vec::new();
                std::mem::swap(&mut attributes, &mut ret.attributes);

                ret = ExpressionWithoutBlock {
                    attributes,
                    kind: if let Ok(r#await) = parser.step_parse() {
                        ExpressionWithoutBlockKind::Await(AwaitExpression {
                            expression: Box::new(ret.into_expression()),
                            dot,
                            r#await,
                        })
                    } else if let Ok(index) = parser.step_parse() {
                        ExpressionWithoutBlockKind::TupleIndex(TupleIndexExpression {
                            expression: Box::new(ret.into_expression()),
                            dot,
                            index,
                        })
                    } else if let Ok((name, parameters)) =
                        parser.step(|parser| Ok((parser.parse()?, parser.parse()?)))
                    {
                        ExpressionWithoutBlockKind::MethodCall(MethodCallExpression {
                            function: Box::new(ret.into_expression()),
                            dot,
                            name,
                            parameters,
                        })
                    } else {
                        ExpressionWithoutBlockKind::Field(FieldExpression {
                            expression: Box::new(ret.into_expression()),
                            dot,
                            identifier: parser.parse()?,
                        })
                    },
                };

                continue;
            }

            if let Ok(question) = parser.step_parse() {
                let mut attributes = Vec::new();
                std::mem::swap(&mut attributes, &mut ret.attributes);

                ret = ExpressionWithoutBlock {
                    attributes,
                    kind: ExpressionWithoutBlockKind::Operator(
                        OperatorExpression::ErrorPropagation(ErrorPropagationExpression {
                            expression: Box::new(ret.into_expression()),
                            question,
                        }),
                    ),
                };

                continue;
            }

            if let Ok(r#as) = parser.step_parse() {
                let mut attributes = Vec::new();
                std::mem::swap(&mut attributes, &mut ret.attributes);

                ret = ExpressionWithoutBlock {
                    attributes,
                    kind: ExpressionWithoutBlockKind::Operator(OperatorExpression::TypeCast(
                        TypeCastExpression {
                            expression: Box::new(ret.into_expression()),
                            r#as,
                            r#type: parser.parse()?,
                        },
                    )),
                };

                continue;
            }

            if let Ok(comparison_operator) = parser.step_parse() {
                let mut attributes = Vec::new();
                std::mem::swap(&mut attributes, &mut ret.attributes);

                return Ok(ExpressionWithoutBlock {
                    attributes,
                    kind: ExpressionWithoutBlockKind::Operator(OperatorExpression::Comparison(
                        ComparisonExpression {
                            left: Box::new(ret.into_expression()),
                            operator: comparison_operator,
                            right: parser.parse()?,
                        },
                    )),
                });
            }

            if let Ok(lazy_boolean_operator) = parser.step_parse() {
                let mut attributes = Vec::new();
                std::mem::swap(&mut attributes, &mut ret.attributes);

                return Ok(ExpressionWithoutBlock {
                    attributes,
                    kind: ExpressionWithoutBlockKind::Operator(OperatorExpression::LazyBoolean(
                        LazyBooleanExpression {
                            left: Box::new(ret.into_expression()),
                            operator: lazy_boolean_operator,
                            right: parser.parse()?,
                        },
                    )),
                });
            }

            if let Ok(arithmetic_or_logical_operator) = parser.step_parse() {
                let mut attributes = Vec::new();
                std::mem::swap(&mut attributes, &mut ret.attributes);

                return Ok(ExpressionWithoutBlock {
                    attributes,
                    kind: ExpressionWithoutBlockKind::Operator(
                        OperatorExpression::ArithmeticOrLogical(ArithmeticOrLogicalExpression {
                            left: Box::new(ret.into_expression()),
                            operator: arithmetic_or_logical_operator,
                            right: parser.parse()?,
                        }),
                    ),
                });
            }

            if let Ok(compound_assignment_operator) = parser.step_parse() {
                let mut attributes = Vec::new();
                std::mem::swap(&mut attributes, &mut ret.attributes);

                return Ok(ExpressionWithoutBlock {
                    attributes,
                    kind: ExpressionWithoutBlockKind::Operator(
                        OperatorExpression::CompoundAssignment(CompoundAssignmentExpression {
                            left: Box::new(ret.into_expression()),
                            operator: compound_assignment_operator,
                            right: parser.parse()?,
                        }),
                    ),
                });
            }

            if let Ok(equals) = parser.step_parse() {
                let mut attributes = Vec::new();
                std::mem::swap(&mut attributes, &mut ret.attributes);

                return Ok(ExpressionWithoutBlock {
                    attributes,
                    kind: ExpressionWithoutBlockKind::Operator(OperatorExpression::Assignment(
                        AssignmentExpression {
                            left: Box::new(ret.into_expression()),
                            equals,
                            right: parser.parse()?,
                        },
                    )),
                });
            }

            if let Ok(parameters) = parser.step_parse() {
                let mut attributes = Vec::new();
                std::mem::swap(&mut attributes, &mut ret.attributes);

                ret = ExpressionWithoutBlock {
                    attributes,
                    kind: ExpressionWithoutBlockKind::Call(CallExpression {
                        function: Box::new(ret.into_expression()),
                        parameters: parameters,
                    }),
                };

                continue;
            }

            if let Ok(index) = parser.step(|parser| {
                let group: &'a Group = parser.parse()?;
                if group.delimiter != Delimiter::Bracket {
                    return Err(group.span.start().error("expected `[`"));
                }

                let mut parser = group.parser();
                let index = parser.parse()?;

                if parser.empty() {
                    Ok(index)
                } else {
                    Err(parser.error("unexpected token"))
                }
            }) {
                let mut attributes = Vec::new();
                std::mem::swap(&mut attributes, &mut ret.attributes);

                ret = ExpressionWithoutBlock {
                    attributes,
                    kind: ExpressionWithoutBlockKind::Index(IndexExpression {
                        expression: Box::new(ret.into_expression()),
                        index,
                    }),
                };

                continue;
            }

            return Ok(ret);
        }
    }
}
