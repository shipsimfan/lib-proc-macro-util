use crate::ast::{
    patterns::{IdentifierPattern, LiteralPattern, ReferencePattern, RestPattern, WildcardPattern},
    MacroInvocation,
};

use super::PathPattern;

mod parse;
mod to_static;
mod to_tokens;

/// A pattern without a range
#[derive(Debug, Clone)]
pub enum PatternWithoutRange<'a> {
    /// A literal value
    Literal(LiteralPattern<'a>),

    /// An identifier
    Identifier(IdentifierPattern<'a>),

    /// Match any value
    Wildcard(WildcardPattern),

    /// Match remaining values
    Rest(RestPattern),

    /// A reference to a pattern
    Reference(ReferencePattern<'a>),

    /// A path to an item
    Path(PathPattern<'a>),

    /// The invocation of a macro
    MacroInvocation(MacroInvocation<'a>),
}
