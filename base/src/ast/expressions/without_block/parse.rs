use crate::{
    ast::{
        expressions::{
            AwaitExpression, CallExpression, ComparisonExpression, ErrorPropagationExpression,
            FieldExpression, IndexExpression, MethodCallExpression, OperatorExpression,
            TypeCastExpression,
        },
        ExpressionWithoutBlock, ExpressionWithoutBlockKind,
    },
    tokens::Group,
    Delimiter, Error, Parse, Parser, Result,
};

impl<'a> Parse<'a> for ExpressionWithoutBlock<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut ret = ExpressionWithoutBlock {
            attributes: parser.parse()?,
            kind: parser.parse()?,
        };

        loop {
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
                    return Err(Error::new_at(
                        "index expression must have square brackets",
                        group.span,
                    ));
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
