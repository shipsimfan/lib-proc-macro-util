use crate::{ast::SimplePath, tokens::Group, Token};
use std::borrow::Cow;

mod semi;

mod from;
mod new;
mod parse;
mod to_static;
mod to_tokens;

pub use semi::MacroInvocationSemi;

/// A macro invocation expands a macro at compile time and replaces the invocation with the result
/// of the macro.
#[derive(Debug, Clone)]
pub struct MacroInvocation<'a> {
    /// The path to the macro
    pub path: SimplePath<'a>,

    /// The "!" identifying the macro invocation
    pub exclamation: Token![!],

    /// The body of the macro invocation
    pub group: Cow<'a, Group>,
}
