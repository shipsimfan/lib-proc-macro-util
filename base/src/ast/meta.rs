use crate::{tokens::Group, Error, Generator, Parse, Parser, Result, ToTokens, Token};
use proc_macro::Delimiter;

/// Metadata about a definition
#[derive(Debug, Clone)]
pub struct Meta<'a> {
    /// The `#` identifying the start of the metadata
    pub hash: Token![#],

    /// The group containing the metadata
    pub group: Group<'a>,
}

impl<'a> Parse<'a> for Meta<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let hash = parser.parse()?;
        let group: Group = parser.parse()?;

        if group.delimiter() != Delimiter::Bracket {
            return Err(Error::new_at(
                "meta must be surrounding by brackets",
                group.span(),
            ));
        }

        Ok(Meta { hash, group })
    }
}

impl<'a> ToTokens for Meta<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        self.hash.to_tokens(generator);
        self.group.to_tokens(generator);
    }
}
