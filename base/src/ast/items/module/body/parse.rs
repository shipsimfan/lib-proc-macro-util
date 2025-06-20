use crate::{ast::items::ModuleBody, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for ModuleBody<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(semi) = parser.step_parse() {
            return Ok(ModuleBody::None(semi));
        }

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Brace {
            return Err(group.span.start().error("expected `{`"));
        }

        let mut parser = group.parser();

        Ok(ModuleBody::Some {
            attributes: parser.parse()?,
            items: parser.parse()?,
        })
    }
}
