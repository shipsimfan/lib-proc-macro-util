use crate::ast::{
    patterns::{IdentifierPattern, LiteralPattern, WildcardPattern},
    MacroInvocation,
};

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

    /// The invocation of a macro
    MacroInvocation(MacroInvocation<'a>),
}
