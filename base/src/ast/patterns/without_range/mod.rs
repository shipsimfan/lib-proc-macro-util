use crate::ast::{
    patterns::{
        GroupedPattern, IdentifierPattern, LiteralPattern, PathPattern, ReferencePattern,
        RestPattern, SlicePattern, StructPattern, TuplePattern, TupleStructPattern,
        WildcardPattern,
    },
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

    /// Match remaining values
    Rest(RestPattern),

    /// A reference to a pattern
    Reference(ReferencePattern<'a>),

    /// A structure
    Struct(StructPattern<'a>),

    /// A tuple structure
    TupleStruct(TupleStructPattern<'a>),

    /// A tuple of multiple patterns
    Tuple(TuplePattern<'a>),

    /// A pattern surrounded by parentheses
    Grouped(GroupedPattern<'a>),

    /// A slice of patterns
    Slice(SlicePattern<'a>),

    /// A path to an item
    Path(PathPattern<'a>),

    /// The invocation of a macro
    MacroInvocation(MacroInvocation<'a>),
}
