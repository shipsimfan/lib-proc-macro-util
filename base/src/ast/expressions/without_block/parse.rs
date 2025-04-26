use crate::{
    ast::{
        expressions::{CallExpression, FieldExpression, MethodCallExpression},
        ExpressionWithoutBlock, ExpressionWithoutBlockKind,
    },
    Parse, Parser, Result, Token,
};

impl<'a> Parse<'a> for ExpressionWithoutBlock<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut ret = ExpressionWithoutBlock {
            attributes: parser.parse()?,
            kind: parser.parse()?,
        };

        loop {
            if parser.peek::<Token![.]>() {
                let mut attributes = Vec::new();
                std::mem::swap(&mut attributes, &mut ret.attributes);

                let dot = parser.parse()?;

                ret = ExpressionWithoutBlock {
                    attributes,
                    kind: if let Ok((name, parameters)) =
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

            return Ok(ret);
        }
    }
}
