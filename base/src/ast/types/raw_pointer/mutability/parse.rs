use crate::{ast::types::RawPointerTypeMutability, Parse, Parser, Result};

impl<'a> Parse<'a> for RawPointerTypeMutability {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(r#mut) = parser.step_parse() {
            return Ok(RawPointerTypeMutability::Mut(r#mut));
        }

        if let Ok(r#const) = parser.step_parse() {
            return Ok(RawPointerTypeMutability::Const(r#const));
        }

        Err(parser.error("expected `mut` or `const`"))
    }
}
