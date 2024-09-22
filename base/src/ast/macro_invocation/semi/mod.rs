use crate::{
    ast::{DelimTokenTree, SimplePath},
    Token,
};

mod from;
mod new;
mod parse;
mod to_tokens;

/// A macro invocation that can appear in an [`Item`] or [`Statement`]
#[derive(Debug, Clone)]
pub enum MacroInvocationSemi<'a> {
    /// The body is surrounded by parentheses or brackets
    ParenthesesOrBracket(SimplePath<'a>, Token![!], DelimTokenTree<'a>, Token![;]),

    /// The body is surrounded by braces
    Brace(SimplePath<'a>, Token![!], DelimTokenTree<'a>),
}
