use crate::{
    ast::TraitBound, supported_languages::*, tokens::Group, Delimiter, Parse, Parser, Result,
};
use i18n::translation::m;

i18n::translation::message_key!(EXPECTED_TRAIT_BOUND [
    EN => { "expected a trait bound" },
    FR => { "une contrainte de trait était attendue" },
    ZH => { "预期的特征约束" },
]);

impl<'a> Parse<'a> for TraitBound<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut group = match parser.step(Parser::parse::<&'a Group>).ok() {
            Some(group) => {
                if group.delimiter != Delimiter::Parenthesis {
                    return Err(parser.error(m!(EXPECTED_TRAIT_BOUND)));
                }

                Some(group.parser())
            }
            None => None,
        };

        let delimited = group.is_some();
        let parser = group.as_mut().unwrap_or(parser);

        Ok(TraitBound {
            delimited,
            question: parser.parse()?,
            for_lifetimes: parser.parse()?,
            path: parser.parse()?,
        })
    }
}
