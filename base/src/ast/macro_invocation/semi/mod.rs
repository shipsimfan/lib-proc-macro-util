use crate::{ast::SimplePath, tokens::Group, Token};
use std::borrow::Cow;

mod from;
mod new;
mod parse;
mod to_static;
mod to_tokens;

/// A macro invocation that can appear in an [`Item`] or [`Statement`]
#[derive(Debug, Clone)]
pub enum MacroInvocationSemi<'a> {
    /// The body is surrounded by parentheses or brackets
    ParenthesesOrBracket(SimplePath<'a>, Token![!], Cow<'a, Group>, Token![;]),

    /// The body is surrounded by braces
    Brace(SimplePath<'a>, Token![!], Cow<'a, Group>),
}
