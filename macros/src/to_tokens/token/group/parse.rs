use super::Group;
use crate::to_tokens::ToTokens;
use proc_macro_util_base::{tokens::Identifier, Result};

impl<'a> Group<'a> {
    pub fn parse(
        group: &'a proc_macro_util_base::tokens::Group,
        generator: &'a Identifier,
    ) -> Result<Self> {
        Ok(Group {
            delimiter: group.delimiter,
            tokens: ToTokens::parse_without_name(&mut group.parser(), generator)?,
        })
    }
}
