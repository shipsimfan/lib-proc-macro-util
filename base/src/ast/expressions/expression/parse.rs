use crate::{
    ast::{expressions::CallExpression, Expression, ExpressionKind},
    Parse, Parser, Result,
};

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut ret = Expression {
            attributes: parser.parse()?,
            kind: parser.parse()?,
        };

        while let Ok(parameters) = parser.step_parse() {
            let mut attributes = Vec::new();
            std::mem::swap(&mut attributes, &mut ret.attributes);

            ret = Expression {
                attributes,
                kind: ExpressionKind::Call(CallExpression {
                    function: Box::new(ret),
                    parameters: parameters,
                }),
            }
        }

        Ok(ret)
    }
}
