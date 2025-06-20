use crate::{ast::MacroInvocationSemi, tokens::Group, Delimiter, Parse, Parser, Result};
use std::borrow::Cow;

impl<'a> Parse<'a> for MacroInvocationSemi<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let path = parser.parse()?;
        let exclamation = parser.parse()?;
        let group: Cow<'a, Group> = parser.parse()?;
        match group.delimiter {
            Delimiter::Parenthesis | Delimiter::Bracket => {
                Ok(MacroInvocationSemi::ParenthesesOrBracket(
                    path,
                    exclamation,
                    group,
                    parser.parse()?,
                ))
            }
            Delimiter::Brace => Ok(MacroInvocationSemi::Brace(path, exclamation, group)),
            Delimiter::None => unimplemented!(),
        }
    }
}
