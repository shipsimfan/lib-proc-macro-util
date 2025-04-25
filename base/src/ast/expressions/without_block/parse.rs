use crate::{
    ast::{expressions::CallExpression, ExpressionWithoutBlock, ExpressionWithoutBlockKind},
    Parse, Parser, Result,
};

impl<'a> Parse<'a> for ExpressionWithoutBlock<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut ret = ExpressionWithoutBlock {
            attributes: parser.parse()?,
            kind: parser.parse()?,
        };

        while let Ok(parameters) = parser.step_parse() {
            let mut attributes = Vec::new();
            std::mem::swap(&mut attributes, &mut ret.attributes);

            ret = ExpressionWithoutBlock {
                attributes,
                kind: ExpressionWithoutBlockKind::Call(CallExpression {
                    function: Box::new(ret.into_expression()),
                    parameters: parameters,
                }),
            }
        }

        Ok(ret)
    }
}
