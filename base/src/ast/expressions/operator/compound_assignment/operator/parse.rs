use crate::{ast::expressions::CompoundAssignmentOperator, Error, Parse, Parser, Result};

impl<'a> Parse<'a> for CompoundAssignmentOperator {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(add) = parser.step_parse() {
            return Ok(CompoundAssignmentOperator::Add(add));
        }

        if let Ok(sub) = parser.step_parse() {
            return Ok(CompoundAssignmentOperator::Sub(sub));
        }

        if let Ok(mul) = parser.step_parse() {
            return Ok(CompoundAssignmentOperator::Mul(mul));
        }

        if let Ok(div) = parser.step_parse() {
            return Ok(CompoundAssignmentOperator::Div(div));
        }

        if let Ok(r#mod) = parser.step_parse() {
            return Ok(CompoundAssignmentOperator::Mod(r#mod));
        }

        if let Ok(and) = parser.step_parse() {
            return Ok(CompoundAssignmentOperator::And(and));
        }

        if let Ok(or) = parser.step_parse() {
            return Ok(CompoundAssignmentOperator::Or(or));
        }

        if let Ok(xor) = parser.step_parse() {
            return Ok(CompoundAssignmentOperator::Xor(xor));
        }

        if let Ok(shl) = parser.step_parse() {
            return Ok(CompoundAssignmentOperator::Shl(shl));
        }

        if let Ok(shr) = parser.step_parse() {
            return Ok(CompoundAssignmentOperator::Shr(shr));
        }

        Err(Error::new_at("unexpected token", parser.span()))
    }
}
