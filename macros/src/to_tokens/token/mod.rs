use group::Group;
use proc_macro_util_base::tokens::{Identifier, Literal, Punctuation};

mod group;
mod parse;
mod to_tokens;

/// A single source token to be generated
#[derive(Debug, Clone)]
pub enum Token<'a> {
    /// A group where [`to_tokens`](crate::to_tokens) needs to be called again
    Group(Group<'a>),

    #[allow(missing_docs)]
    Identifier(&'a Identifier),

    #[allow(missing_docs)]
    Literal(&'a Literal),

    #[allow(missing_docs)]
    Punctuation(&'a Punctuation),

    /// The token is a variable to be generated
    Variable(&'a Identifier),
}
