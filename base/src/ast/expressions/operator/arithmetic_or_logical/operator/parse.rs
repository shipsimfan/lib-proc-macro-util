use crate::{ast::expressions::ArithmeticOrLogicalOperator, Parse, Parser, Result};

impl<'a> Parse<'a> for ArithmeticOrLogicalOperator {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(add) = parser.step_parse() {
            return Ok(ArithmeticOrLogicalOperator::Add(add));
        }

        if let Ok(sub) = parser.step_parse() {
            return Ok(ArithmeticOrLogicalOperator::Sub(sub));
        }

        if let Ok(mul) = parser.step_parse() {
            return Ok(ArithmeticOrLogicalOperator::Mul(mul));
        }

        if let Ok(div) = parser.step_parse() {
            return Ok(ArithmeticOrLogicalOperator::Div(div));
        }

        if let Ok(r#mod) = parser.step_parse() {
            return Ok(ArithmeticOrLogicalOperator::Mod(r#mod));
        }

        if let Ok(and) = parser.step_parse() {
            return Ok(ArithmeticOrLogicalOperator::And(and));
        }

        if let Ok(or) = parser.step_parse() {
            return Ok(ArithmeticOrLogicalOperator::Or(or));
        }

        if let Ok(xor) = parser.step_parse() {
            return Ok(ArithmeticOrLogicalOperator::Xor(xor));
        }

        if let Ok(shl) = parser.step_parse() {
            return Ok(ArithmeticOrLogicalOperator::Shl(shl));
        }

        if let Ok(shr) = parser.step_parse() {
            return Ok(ArithmeticOrLogicalOperator::Shr(shr));
        }

        Err(parser.error("unexpected token"))
    }
}
