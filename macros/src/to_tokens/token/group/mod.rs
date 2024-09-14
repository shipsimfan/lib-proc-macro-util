use crate::to_tokens::ToTokens;
use proc_macro_util_base::Delimiter;

mod parse;
mod to_tokens;

/// A group which will be generated into a sub-generator
#[derive(Debug, Clone)]
pub struct Group<'a> {
    /// The delimiters surrounding this group
    delimiter: Delimiter,

    /// The tokes inside this group
    tokens: ToTokens<'a>,
}
