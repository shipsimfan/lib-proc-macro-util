use crate::{ast::expressions::ArrayElements, Parse, Parser, Result};

impl<'a> Parse<'a> for ArrayElements<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let first = parser.parse()?;

        if let Ok(semi) = parser.step_parse() {
            Ok(ArrayElements::Uniform {
                element: first,
                semi,
                count: parser.parse()?,
            })
        } else {
            Ok(ArrayElements::Every {
                first,
                remaining: parser.parse()?,
                last: parser.parse()?,
            })
        }
    }
}
