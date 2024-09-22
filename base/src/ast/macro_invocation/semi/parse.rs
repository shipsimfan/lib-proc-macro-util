use crate::{ast::MacroInvocationSemi, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for MacroInvocationSemi<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let path = parser.parse()?;
        let exclamation = parser.parse()?;
        let group: &'a Group = parser.parse()?;
        match group.delimiter {
            Delimiter::Parenthesis | Delimiter::Bracket => {
                Ok(MacroInvocationSemi::ParenthesesOrBracket(
                    path,
                    exclamation,
                    group.into(),
                    parser.parse()?,
                ))
            }
            Delimiter::Brace => Ok(MacroInvocationSemi::Brace(path, exclamation, group.into())),
            Delimiter::None => unimplemented!(),
        }
    }
}
